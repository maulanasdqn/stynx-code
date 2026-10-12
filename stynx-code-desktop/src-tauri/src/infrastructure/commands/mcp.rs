use tauri::State;

use crate::application::mcp::{config, registry};
use crate::domain::ide_mcp::{McpEntry, McpInstalled, McpPage};
use crate::infrastructure::ide_state::IdeState;

#[tauri::command]
pub async fn mcp_search(query: String, cursor: Option<String>) -> Result<McpPage, String> {
    registry::search(&query, cursor.as_deref()).await
}

#[tauri::command]
pub async fn mcp_installed(ide: State<'_, IdeState>) -> Result<Vec<McpInstalled>, String> {
    Ok(config::installed(ide.root().await?.path()))
}

#[tauri::command]
pub async fn mcp_install(ide: State<'_, IdeState>, scope: String, name: String, entry: McpEntry) -> Result<(), String> {
    config::install(&scope, ide.root().await?.path(), &name, &entry)
}

#[tauri::command]
pub async fn mcp_uninstall(ide: State<'_, IdeState>, scope: String, name: String) -> Result<(), String> {
    config::uninstall(&scope, ide.root().await?.path(), &name)
}
