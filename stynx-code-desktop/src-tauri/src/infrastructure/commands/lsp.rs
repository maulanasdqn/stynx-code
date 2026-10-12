use serde_json::Value;
use tauri::State;

use crate::application::lsp::manager::Target;
use crate::domain::ide_lsp::LspLocation;
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
