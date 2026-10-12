use super::*;
use serde_json::json;

#[test]
fn npm_package_with_env_and_positional_args() {
    let server = json!({
        "name": "io.github.acme/githits", "version": "0.28.0", "description": "Search",
        "packages": [{ "registryType": "npm", "identifier": "githits", "version": "0.28.0",
            "transport": { "type": "stdio" },
            "environmentVariables": [{ "name": "GITHUB_TOKEN", "isRequired": true, "isSecret": true }],
            "packageArguments": [{ "type": "positional", "value": "mcp" }, { "type": "positional", "valueHint": "repo_path" }] }]
    });
    let item = listing(&server).unwrap();
    assert_eq!(item.name, "githits");
    let plan = item.plan.unwrap();
    assert_eq!((plan.kind.as_str(), plan.command.as_str()), ("npm", "npx"));
    assert_eq!(plan.args, ["-y", "githits@0.28.0", "mcp", "{repo_path}"]);
    let keys: Vec<_> = plan.inputs.iter().map(|i| (i.key.as_str(), i.target.as_str(), i.secret)).collect();
    assert_eq!(keys, [("GITHUB_TOKEN", "env", true), ("repo_path", "arg", false)]);
}

#[test]
fn remote_server_uses_mcp_remote_with_header_inputs() {
    let server = json!({
        "name": "ai.smithery/obsidian", "version": "0.4.0",
        "remotes": [{ "type": "streamable-http", "url": "https://x.example/mcp",
            "headers": [{ "name": "Authorization", "value": "Bearer {smithery_api_key}", "isSecret": true }] }]
    });
    let plan = listing(&server).unwrap().plan.unwrap();
    assert_eq!(plan.kind, "remote");
    assert_eq!(plan.args, ["-y", "mcp-remote", "https://x.example/mcp", "--header", "Authorization: Bearer {smithery_api_key}"]);
    assert_eq!(plan.inputs[0].key, "smithery_api_key");
}

#[test]
fn short_names_are_config_safe() {
    assert_eq!(short_name("io.github.foo/Bar MCP"), "bar-mcp");
}

#[tokio::test]
#[ignore = "hits the live MCP registry"]
async fn live_search_returns_installable_servers() {
    let page = search("github", None).await.unwrap();
    assert!(!page.servers.is_empty());
    let installable = page.servers.iter().filter(|s| s.plan.is_some()).count();
    println!("{} servers, {installable} installable, first: {:?}", page.servers.len(), page.servers[0].plan);
    assert!(installable > 0);
}
