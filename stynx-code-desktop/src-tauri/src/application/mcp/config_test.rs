use super::*;

#[test]
fn install_keeps_other_keys_and_uninstall_removes() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(".mcp.json"), r#"{"other": 1, "mcpServers": {"keep": {"command": "x"}}}"#).unwrap();
    let entry = McpEntry { command: "npx".into(), args: vec!["-y".into(), "pkg".into()], env: [("K".to_string(), "v".to_string())].into() };
    install("workspace", dir.path(), "new", &entry).unwrap();
    let names: Vec<_> = installed(dir.path()).into_iter().filter(|i| i.scope == "workspace").map(|i| i.name).collect();
    assert_eq!(names, ["keep", "new"]);
    let raw: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(dir.path().join(".mcp.json")).unwrap()).unwrap();
    assert_eq!(raw["other"], 1);
    assert_eq!(raw["mcpServers"]["new"]["env"]["K"], "v");
    uninstall("workspace", dir.path(), "new").unwrap();
    assert!(installed(dir.path()).iter().all(|i| i.name != "new"));
}
