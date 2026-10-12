use tauri::{AppHandle, Emitter, State};

use crate::application::session::SessionSlot;
use crate::application::workspace::{fs_ops, index, root::WorkspaceRoot, watch::WorkspaceWatcher};
use crate::domain::ide_fs::{FileContent, FileNode, FsChange, SaveResult};
use crate::infrastructure::commands::active;
use crate::infrastructure::emitter::IDE_FS_EVENT;
use crate::infrastructure::ide_state::{IdeState, IdeWorkspace};

#[tauri::command]
pub async fn ide_attach(
    app: AppHandle,
    slot: State<'_, SessionSlot>,
    ide: State<'_, IdeState>,
) -> Result<String, String> {
    let session = active(&slot).await?;
    let root = WorkspaceRoot::new(&session.workspace_path)?;
    let emitter = app.clone();
    let watcher = WorkspaceWatcher::start(root.clone(), move |paths| {
        let _ = emitter.emit(IDE_FS_EVENT, FsChange { paths });
    })
    .map_err(|error| tracing::warn!("file watcher unavailable: {error}"))
    .ok();
    let display = root.path().to_string_lossy().to_string();
    *ide.0.lock().await = Some(IdeWorkspace { root, _watcher: watcher });
    Ok(display)
}

#[tauri::command]
pub async fn ide_list_dir(ide: State<'_, IdeState>, path: String) -> Result<Vec<FileNode>, String> {
    fs_ops::list_dir(&ide.root().await?, &path)
}

#[tauri::command]
pub async fn ide_read(ide: State<'_, IdeState>, path: String) -> Result<FileContent, String> {
    let root = ide.root().await?;
    tokio::task::spawn_blocking(move || fs_ops::read(&root, &path)).await.map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn ide_save(
    ide: State<'_, IdeState>,
    path: String,
    content: String,
    fingerprint: String,
) -> Result<SaveResult, String> {
    let root = ide.root().await?;
    tokio::task::spawn_blocking(move || fs_ops::save(&root, &path, &content, &fingerprint))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn ide_create(ide: State<'_, IdeState>, path: String, is_dir: bool) -> Result<(), String> {
    fs_ops::create(&ide.root().await?, &path, is_dir)
}

#[tauri::command]
pub async fn ide_rename(ide: State<'_, IdeState>, from: String, to: String) -> Result<(), String> {
    fs_ops::rename(&ide.root().await?, &from, &to)
}

#[tauri::command]
pub async fn ide_delete(ide: State<'_, IdeState>, path: String) -> Result<(), String> {
    fs_ops::delete(&ide.root().await?, &path)
}

#[tauri::command]
pub async fn ide_index(ide: State<'_, IdeState>) -> Result<Vec<String>, String> {
    let root = ide.root().await?;
    tokio::task::spawn_blocking(move || index::index_files(&root)).await.map_err(|error| error.to_string())
}
