use super::*;
use std::time::Duration;

#[tokio::test]
#[ignore = "needs rust-analyzer; run with --ignored"]
async fn rust_analyzer_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let root = std::fs::canonicalize(dir.path()).unwrap();
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n").unwrap();
    let text = "fn helper() -> u32 { 1 }\nfn main() {\n    let x: u32 = \"nope\";\n    helper();\n}\n";
    std::fs::write(root.join("src/main.rs"), text).unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<LspEvent>();
    let path_var = std::env::var("STYNX_LSP_PATH").ok().or_else(|| crate::application::env_path::login_path().map(str::to_string));
    let mut manager = LspManager::new(root.clone(), path_var, Arc::new(move |event| { let _ = tx.send(event); }));
    if !manager.open("src/main.rs", text).await.unwrap() {
        let mut seen = Vec::new();
        while let Ok(event) = rx.try_recv() {
            seen.push(format!("{event:?}"));
        }
        panic!("rust-analyzer did not start: {seen:?}");
    }
    let diagnostics = tokio::time::timeout(Duration::from_secs(120), async {
        while let Some(event) = rx.recv().await {
            if let LspEvent::Diagnostics { path, diagnostics } = event
                && path == "src/main.rs"
                && diagnostics.as_array().is_some_and(|list| !list.is_empty())
            {
                return diagnostics;
            }
        }
        Value::Null
    })
    .await
    .expect("diagnostics in time");
    assert!(diagnostics.to_string().contains("mismatched") || diagnostics.to_string().contains("expected"), "{diagnostics}");
    let target = manager.target("src/main.rs").await.unwrap();
    let hover = target.request_at("textDocument/hover", 3, 5).await.unwrap();
    assert!(hover.to_string().contains("helper"), "{hover}");
    let defs = target.definition(3, 5).await.unwrap();
    assert_eq!(defs[0].path, "src/main.rs");
    assert_eq!(defs[0].range["start"]["line"], 0);
    manager.shutdown_all().await;
}
