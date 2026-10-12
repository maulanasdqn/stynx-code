use std::path::PathBuf;

use tauri::State;

use crate::application::git::{diff, ops, status};
use crate::domain::ide_git::{GitDiff, GitStatus};
use crate::infrastructure::ide_state::IdeState;

async fn blocking<T, F>(ide: &IdeState, work: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce(PathBuf) -> Result<T, String> + Send + 'static,
{
    let root = ide.root().await?.path().to_path_buf();
    tokio::task::spawn_blocking(move || work(root)).await.map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn git_status(ide: State<'_, IdeState>) -> Result<GitStatus, String> {
    blocking(&ide, |root| status::status(&root)).await
}

#[tauri::command]
pub async fn git_diff(ide: State<'_, IdeState>, path: String, staged: bool) -> Result<GitDiff, String> {
    blocking(&ide, move |root| diff::diff(&root, &path, staged)).await
}

#[tauri::command]
pub async fn git_stage(ide: State<'_, IdeState>, paths: Vec<String>) -> Result<(), String> {
    blocking(&ide, move |root| ops::stage(&root, &paths)).await
}

#[tauri::command]
pub async fn git_unstage(ide: State<'_, IdeState>, paths: Vec<String>) -> Result<(), String> {
    blocking(&ide, move |root| ops::unstage(&root, &paths)).await
}

#[tauri::command]
pub async fn git_discard(ide: State<'_, IdeState>, paths: Vec<String>) -> Result<(), String> {
    blocking(&ide, move |root| ops::discard(&root, &paths)).await
}

#[tauri::command]
pub async fn git_commit(ide: State<'_, IdeState>, message: String) -> Result<(), String> {
    blocking(&ide, move |root| ops::commit(&root, &message)).await
}
