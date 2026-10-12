use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use crate::domain::ide_mcp::{McpEntry, McpInstalled};

pub fn config_path(scope: &str, workspace: &Path) -> Result<PathBuf, String> {
    match scope {
        "workspace" => Ok(workspace.join(".mcp.json")),
        "user" => std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".claude").join("mcp.json")).ok_or("no home directory".into()),
        _ => Err(format!("unknown scope {scope}")),
    }
}

fn read(path: &Path) -> Result<Map<String, Value>, String> {
    match std::fs::read_to_string(path) {
        Ok(text) if text.trim().is_empty() => Ok(Map::new()),
        Ok(text) => match serde_json::from_str::<Value>(&text).map_err(|e| format!("{}: {e}", path.display()))? {
            Value::Object(map) => Ok(map),
            _ => Err(format!("{} is not a JSON object", path.display())),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Map::new()),
        Err(error) => Err(error.to_string()),
    }
}

fn write(path: &Path, map: Map<String, Value>) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| e.to_string())?;
    std::fs::write(path, text + "\n").map_err(|e| e.to_string())
}

pub fn installed(workspace: &Path) -> Vec<McpInstalled> {
    let mut out = Vec::new();
    for scope in ["workspace", "user"] {
        let Ok(path) = config_path(scope, workspace) else { continue };
        let Ok(map) = read(&path) else { continue };
        for (name, value) in map.get("mcpServers").and_then(Value::as_object).cloned().unwrap_or_default() {
            if let Ok(entry) = serde_json::from_value::<McpEntry>(value) {
                out.push(McpInstalled { name, scope: scope.into(), entry });
            }
        }
    }
    out
}

pub fn install(scope: &str, workspace: &Path, name: &str, entry: &McpEntry) -> Result<(), String> {
    if name.trim().is_empty() || entry.command.trim().is_empty() {
        return Err("server name and command are required".into());
    }
    let path = config_path(scope, workspace)?;
    let mut map = read(&path)?;
    let servers = map.entry("mcpServers").or_insert_with(|| Value::Object(Map::new()));
    let servers = servers.as_object_mut().ok_or("mcpServers is not an object")?;
    servers.insert(name.to_string(), serde_json::to_value(entry).map_err(|e| e.to_string())?);
    write(&path, map)
}

pub fn uninstall(scope: &str, workspace: &Path, name: &str) -> Result<(), String> {
    let path = config_path(scope, workspace)?;
    let mut map = read(&path)?;
    if let Some(servers) = map.get_mut("mcpServers").and_then(Value::as_object_mut) {
        servers.remove(name);
    }
    write(&path, map)
}

#[cfg(test)]
#[path = "config_test.rs"]
mod tests;
