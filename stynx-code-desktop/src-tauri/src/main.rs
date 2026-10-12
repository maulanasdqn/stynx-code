#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod domain;
mod infrastructure;

use application::session::SessionSlot;
use tauri::Manager;
use infrastructure::commands::{dialogs, files, fs, terminal, history, messaging, session, settings};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(SessionSlot::default())
        .manage(infrastructure::ide_state::IdeState::default())
        .manage(infrastructure::ide_state::TerminalState::default())
        .invoke_handler(tauri::generate_handler![
            session::init_session,
            session::get_session_info,
            messaging::send_message,
            messaging::cancel,
            messaging::respond_permission,
            messaging::respond_ask_user,
            messaging::respond_workspace_message,
            settings::set_model,
            settings::set_mode,
            settings::set_thinking,
            settings::set_provider_key,
            history::list_sessions,
            history::load_session,
            history::new_session,
            history::delete_session,
            dialogs::pick_folder,
            dialogs::pick_files,
            dialogs::confirm_destructive,
            dialogs::open_external,
            fs::ide_attach,
            fs::ide_list_dir,
            fs::ide_read,
            fs::ide_save,
            fs::ide_create,
            fs::ide_rename,
            fs::ide_delete,
            fs::ide_index,
            terminal::terminal_shells,
            terminal::terminal_open,
            terminal::terminal_write,
            terminal::terminal_resize,
            terminal::terminal_close,
            files::read_file,
            files::read_image,
            files::fetch_reference,
            files::list_project_files,
        ])
        .build(tauri::generate_context!())
        .expect("error while building stynx desktop")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event
                && let Ok(mut terminals) = app.state::<infrastructure::ide_state::TerminalState>().0.lock()
            {
                terminals.close_all();
            }
        });
}
