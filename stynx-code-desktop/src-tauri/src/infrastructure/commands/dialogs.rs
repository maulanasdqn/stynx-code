use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tokio::sync::oneshot;

const IMAGE_EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "gif", "webp"];

#[tauri::command]
pub async fn pick_folder(app: AppHandle, window: WebviewWindow) -> Result<Option<String>, String> {
    let (tx, rx) = oneshot::channel();
    app.dialog()
        .file()
        .set_parent(&window)
        .set_title("Choose a project folder")
        .set_can_create_directories(true)
        .pick_folder(move |path| {
            let _ = tx.send(path);
        });
    let picked = rx.await.map_err(|error| error.to_string())?;
    Ok(picked.and_then(|path| path.into_path().ok()).map(|path| path.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn pick_files(
    app: AppHandle,
    window: WebviewWindow,
    title: String,
    images: bool,
) -> Result<Vec<String>, String> {
    let (tx, rx) = oneshot::channel();
    let mut builder = app.dialog().file().set_parent(&window).set_title(title);
    if images {
        builder = builder.add_filter("Images", &IMAGE_EXTENSIONS);
    }
    builder.pick_files(move |paths| {
        let _ = tx.send(paths);
    });
    let picked = rx.await.map_err(|error| error.to_string())?.unwrap_or_default();
    Ok(picked
        .into_iter()
        .filter_map(|path| path.into_path().ok())
        .map(|path| path.to_string_lossy().to_string())
        .collect())
}

#[tauri::command]
pub async fn confirm_destructive(
    app: AppHandle,
    window: WebviewWindow,
    title: String,
    message: String,
    ok_label: String,
) -> Result<bool, String> {
    let (tx, rx) = oneshot::channel();
    app.dialog()
        .message(message)
        .parent(&window)
        .title(title)
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::OkCancelCustom(ok_label, "Cancel".to_string()))
        .show(move |confirmed| {
            let _ = tx.send(confirmed);
        });
    rx.await.map_err(|error| error.to_string())
}
