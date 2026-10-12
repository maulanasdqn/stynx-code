pub mod dialogs;
pub mod files;
pub mod fs;
pub mod git;
pub mod history;
pub mod lsp;
pub mod mcp;
pub mod messaging;
pub mod session;
pub mod settings;
pub mod terminal;

use std::sync::Arc;

use tauri::State;

use crate::application::session::{Session, SessionSlot};

pub async fn active(slot: &State<'_, SessionSlot>) -> Result<Arc<Session>, String> {
    slot.0
        .lock()
        .await
        .clone()
        .ok_or_else(|| "no active session — call init_session first".to_string())
}
