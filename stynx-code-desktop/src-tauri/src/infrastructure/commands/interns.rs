use crate::application::interns::{InternEntry, delete_at, list_at, save_at, settings_path};
use crate::application::session::current_interns;
use crate::domain::models::InternInfo;

fn path() -> Result<std::path::PathBuf, String> {
    settings_path().ok_or_else(|| "cannot determine home directory".into())
}

#[tauri::command]
pub fn intern_status() -> Vec<InternInfo> {
    current_interns()
}

#[tauri::command]
pub fn intern_configs() -> Result<Vec<InternEntry>, String> {
    list_at(&path()?)
}

#[tauri::command]
pub fn intern_save(entry: InternEntry, previous: Option<String>) -> Result<(), String> {
    save_at(&path()?, &entry, previous.as_deref())
}

#[tauri::command]
pub fn intern_delete(name: String) -> Result<(), String> {
    delete_at(&path()?, &name)
}
