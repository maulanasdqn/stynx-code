use serde_json::Value;
use tauri::{AppHandle, Emitter, State};

use crate::application::lsp::manager::Target;
use crate::application::lsp::catalog::{SERVERS, install_dir, resolve, spec_by_id};
use crate::application::lsp::install::npm;
use crate::domain::ide_lsp::{LspEvent, LspLocation, ServerInfo};
use crate::infrastructure::emitter::IDE_LSP_EVENT;
use crate::infrastructure::ide_state::LspState;

async fn target(lsp: &LspState, path: &str) -> Option<Target> {
    lsp.0.lock().await.as_mut()?.target(path).await
}

#[tauri::command]
pub async fn lsp_open(lsp: State<'_, LspState>, path: String, text: String) -> Result<bool, String> {
    match lsp.0.lock().await.as_mut() {
        Some(manager) => manager.open(&path, &text).await,
        None => Ok(false),
    }
}

#[tauri::command]
pub async fn lsp_change(lsp: State<'_, LspState>, path: String, text: String, version: i64) -> Result<(), String> {
    match lsp.0.lock().await.as_mut() {
        Some(manager) => manager.change(&path, &text, version).await,
        None => Ok(()),
    }
}

#[tauri::command]
pub async fn lsp_save(lsp: State<'_, LspState>, path: String) -> Result<(), String> {
    match lsp.0.lock().await.as_mut() {
        Some(manager) => manager.notify_doc(&path, "textDocument/didSave").await,
        None => Ok(()),
    }
}

#[tauri::command]
pub async fn lsp_close(lsp: State<'_, LspState>, path: String) -> Result<(), String> {
    match lsp.0.lock().await.as_mut() {
        Some(manager) => manager.notify_doc(&path, "textDocument/didClose").await,
        None => Ok(()),
    }
}

#[tauri::command]
pub async fn lsp_completion(lsp: State<'_, LspState>, path: String, line: u32, character: u32) -> Result<Value, String> {
    match target(&lsp, &path).await {
        Some(t) => t.request_at("textDocument/completion", line, character).await,
        None => Ok(Value::Null),
    }
}

#[tauri::command]
pub async fn lsp_resolve(lsp: State<'_, LspState>, path: String, item: Value) -> Result<Value, String> {
    match target(&lsp, &path).await {
        Some(t) => t.resolve(item).await,
        None => Ok(item),
    }
}

#[tauri::command]
pub async fn lsp_hover(lsp: State<'_, LspState>, path: String, line: u32, character: u32) -> Result<Value, String> {
    match target(&lsp, &path).await {
        Some(t) => t.request_at("textDocument/hover", line, character).await,
        None => Ok(Value::Null),
    }
}

#[tauri::command]
pub async fn lsp_definition(
    lsp: State<'_, LspState>,
    path: String,
    line: u32,
    character: u32,
) -> Result<Vec<LspLocation>, String> {
    match target(&lsp, &path).await {
        Some(t) => t.definition(line, character).await,
        None => Ok(Vec::new()),
    }
}

fn path_var() -> Option<String> {
    crate::application::env_path::login_path().map(str::to_string)
}

#[tauri::command]
pub async fn lsp_servers() -> Result<Vec<ServerInfo>, String> {
    let path_var = tokio::task::spawn_blocking(path_var).await.map_err(|e| e.to_string())?;
    let managed_root = install_dir();
    Ok(SERVERS
        .iter()
        .map(|spec| {
            let location = resolve(spec, path_var.as_deref()).map(|(path, _)| path);
            let managed = matches!((&location, &managed_root), (Some(path), Some(root)) if path.starts_with(root));
            ServerInfo {
                id: spec.id.into(),
                name: spec.name.into(),
                extensions: spec.extensions.iter().map(|e| e.to_string()).collect(),
                location: location.map(|p| p.to_string_lossy().to_string()),
                managed,
                installable: !spec.npm.is_empty(),
                hint: spec.hint.into(),
            }
        })
        .collect())
}

async fn change(app: AppHandle, lsp: &LspState, id: &str, uninstall: bool) -> Result<(), String> {
    let spec = spec_by_id(id).ok_or("unknown language server")?;
    let path_var = tokio::task::spawn_blocking(path_var).await.map_err(|e| e.to_string())?;
    let log_id = id.to_string();
    let sink = move |line: String| {
        let _ = app.emit(IDE_LSP_EVENT, LspEvent::InstallLog { id: log_id.clone(), line });
    };
    npm(spec, uninstall, path_var.as_deref(), sink).await?;
    if let Some(manager) = lsp.0.lock().await.as_mut() {
        manager.forget(id).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn lsp_install(app: AppHandle, lsp: State<'_, LspState>, id: String) -> Result<(), String> {
    change(app, &lsp, &id, false).await
}

#[tauri::command]
pub async fn lsp_uninstall(app: AppHandle, lsp: State<'_, LspState>, id: String) -> Result<(), String> {
    change(app, &lsp, &id, true).await
}

const DOCUMENT_METHODS: &[&str] = &[
    "textDocument/formatting",
    "textDocument/rangeFormatting",
    "textDocument/documentSymbol",
    "textDocument/signatureHelp",
];

#[tauri::command]
pub async fn lsp_request(lsp: State<'_, LspState>, path: String, method: String, params: Value) -> Result<Value, String> {
    if !DOCUMENT_METHODS.contains(&method.as_str()) {
        return Err(format!("{method} is not allowed"));
    }
    match target(&lsp, &path).await {
        Some(t) => t.request_doc(&method, params).await,
        None => Ok(Value::Null),
    }
}
