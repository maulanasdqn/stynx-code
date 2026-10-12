use serde_json::Value;

use crate::domain::ide_mcp::{McpInput, McpListing, McpPage, McpPlan};

const REGISTRY: &str = "https://registry.modelcontextprotocol.io/v0/servers";

pub async fn search(query: &str, cursor: Option<&str>) -> Result<McpPage, String> {
    let mut request = reqwest::Client::new()
        .get(REGISTRY)
        .query(&[("limit", "30"), ("version", "latest")])
        .timeout(std::time::Duration::from_secs(20));
    if !query.trim().is_empty() {
        request = request.query(&[("search", query.trim())]);
    }
    if let Some(cursor) = cursor {
        request = request.query(&[("cursor", cursor)]);
    }
    let body: Value = request.send().await.map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;
    let servers = body["servers"].as_array().map(|list| list.iter().filter_map(|x| listing(&x["server"])).collect()).unwrap_or_default();
    let next_cursor = body["metadata"]["nextCursor"].as_str().map(str::to_string);
    Ok(McpPage { servers, next_cursor })
}

fn text(value: &Value, key: &str) -> String {
    value[key].as_str().unwrap_or_default().to_string()
}

pub fn short_name(id: &str) -> String {
    let tail = id.rsplit(['/', '.']).next().unwrap_or(id);
    let clean: String = tail.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).collect();
    clean.trim_matches('-').to_lowercase()
}

pub fn listing(server: &Value) -> Option<McpListing> {
    let id = server["name"].as_str()?.to_string();
    let title = server["title"].as_str().map(str::to_string).unwrap_or_else(|| short_name(&id));
    Some(McpListing {
        name: short_name(&id),
        title,
        description: text(server, "description"),
        version: text(server, "version"),
        repository: server["repository"]["url"].as_str().map(str::to_string),
        website: server["websiteUrl"].as_str().map(str::to_string),
        plan: plan(server),
        id,
    })
}

fn env_inputs(list: &Value) -> Vec<McpInput> {
    list.as_array()
        .map(|vars| {
            vars.iter()
                .filter_map(|v| {
                    let name = v["name"].as_str()?.to_string();
                    Some(McpInput {
                        label: name.clone(),
                        description: text(v, "description"),
                        required: v["isRequired"].as_bool().unwrap_or(false),
                        secret: v["isSecret"].as_bool().unwrap_or(false),
                        default: v["default"].as_str().map(str::to_string),
                        target: "env".into(),
                        key: name,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn arguments(list: &Value, args: &mut Vec<String>, inputs: &mut Vec<McpInput>) {
    for arg in list.as_array().into_iter().flatten() {
        let named = arg["type"] == "named";
        if named && let Some(name) = arg["name"].as_str() {
            args.push(name.to_string());
        }
        match arg["value"].as_str().or_else(|| arg["default"].as_str()) {
            Some(value) => args.push(value.to_string()),
            None => {
                let key = arg["valueHint"].as_str().or_else(|| arg["name"].as_str()).unwrap_or("value").trim_start_matches('-').to_string();
                args.push(format!("{{{key}}}"));
                inputs.push(McpInput {
                    label: key.clone(),
                    description: text(arg, "description"),
                    required: arg["isRequired"].as_bool().unwrap_or(true),
                    secret: arg["isSecret"].as_bool().unwrap_or(false),
                    default: None,
                    target: "arg".into(),
                    key,
                });
            }
        }
    }
}

fn plan(server: &Value) -> Option<McpPlan> {
    let packages = server["packages"].as_array().cloned().unwrap_or_default();
    let stdio = packages.iter().find(|p| p["transport"]["type"].as_str().unwrap_or("stdio") == "stdio");
    if let Some(pkg) = stdio {
        let id = pkg["identifier"].as_str()?;
        let version = pkg["version"].as_str().unwrap_or("latest");
        let mut inputs = env_inputs(&pkg["environmentVariables"]);
        let (kind, command, mut args) = match pkg["registryType"].as_str()? {
            "npm" => ("npm", "npx", vec!["-y".to_string(), format!("{id}@{version}")]),
            "pypi" => ("pypi", "uvx", vec![format!("{id}@{version}")]),
            "oci" => {
                let mut run = vec!["run".to_string(), "-i".into(), "--rm".into()];
                for input in &inputs {
                    run.extend(["-e".into(), input.key.clone()]);
                }
                let image = if id.contains(':') { id.to_string() } else { format!("{id}:{version}") };
                run.push(image);
                ("oci", "docker", run)
            }
            _ => return None,
        };
        arguments(&pkg["packageArguments"], &mut args, &mut inputs);
        return Some(McpPlan { kind: kind.into(), command: command.into(), args, inputs });
    }
    let remote = server["remotes"].as_array()?.first()?;
    let mut args = vec!["-y".to_string(), "mcp-remote".into(), remote["url"].as_str()?.to_string()];
    let mut inputs = Vec::new();
    for header in remote["headers"].as_array().into_iter().flatten() {
        let name = header["name"].as_str()?;
        let template = header["value"].as_str().map(str::to_string).unwrap_or_else(|| format!("{{{}}}", name.to_lowercase()));
        for key in template.split('{').skip(1).filter_map(|part| part.split_once('}').map(|(k, _)| k.to_string())) {
            inputs.push(McpInput {
                label: key.clone(),
                description: text(header, "description"),
                required: header["isRequired"].as_bool().unwrap_or(true),
                secret: header["isSecret"].as_bool().unwrap_or(true),
                default: None,
                target: "arg".into(),
                key,
            });
        }
        args.extend(["--header".to_string(), format!("{name}: {template}")]);
    }
    Some(McpPlan { kind: "remote".into(), command: "npx".into(), args, inputs })
}
#[cfg(test)]
#[path = "registry_test.rs"]
mod tests;
