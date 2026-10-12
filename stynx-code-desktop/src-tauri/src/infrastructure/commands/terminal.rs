use serde::Serialize;
use tauri::State;
use tauri::ipc::Channel;

use crate::application::terminal::shells::available_shells;
use crate::infrastructure::ide_state::{IdeState, TerminalState};

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TerminalEvent {
    Output { data: String },
    Exit,
}

#[tauri::command]
pub fn terminal_shells() -> Vec<String> {
    available_shells()
}

#[tauri::command]
pub async fn terminal_open(
    ide: State<'_, IdeState>,
    terminals: State<'_, TerminalState>,
    shell: Option<String>,
    cols: u16,
    rows: u16,
    on_event: Channel<TerminalEvent>,
) -> Result<u32, String> {
    let root = ide.root().await?;
    let output = on_event.clone();
    let mut manager = terminals.0.lock().map_err(|e| e.to_string())?;
    manager.open(
        shell,
        root.path(),
        cols,
        rows,
        move |data| {
            let _ = output.send(TerminalEvent::Output { data });
        },
        move || {
            let _ = on_event.send(TerminalEvent::Exit);
        },
    )
}

#[tauri::command]
pub fn terminal_write(terminals: State<'_, TerminalState>, id: u32, data: String) -> Result<(), String> {
    terminals.0.lock().map_err(|e| e.to_string())?.write(id, &data)
}

#[tauri::command]
pub fn terminal_resize(terminals: State<'_, TerminalState>, id: u32, cols: u16, rows: u16) -> Result<(), String> {
    terminals.0.lock().map_err(|e| e.to_string())?.resize(id, cols, rows)
}

#[tauri::command]
pub fn terminal_close(terminals: State<'_, TerminalState>, id: u32) -> Result<(), String> {
    terminals.0.lock().map_err(|e| e.to_string())?.close(id);
    Ok(())
}
