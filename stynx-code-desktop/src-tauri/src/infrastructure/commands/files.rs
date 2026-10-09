use serde::Serialize;

const MAX_FILE_BYTES: usize = 512 * 1024;
const MAX_FETCH_BYTES: usize = 2 * 1024 * 1024;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchedReference {
    pub content_type: String,
    pub body: String,
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    let bytes = tokio::fs::read(&path).await.map_err(|error| error.to_string())?;
    let capped = &bytes[..bytes.len().min(MAX_FILE_BYTES)];
    Ok(String::from_utf8_lossy(capped).to_string())
}

const MAX_IMAGE_BYTES: usize = 20 * 1024 * 1024;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFile {
    pub media_type: String,
    pub data: String,
}

#[tauri::command]
pub async fn read_image(path: String) -> Result<ImageFile, String> {
    use base64::Engine;
    let bytes = tokio::fs::read(&path).await.map_err(|error| error.to_string())?;
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err("image is larger than 20 MB".to_string());
    }
    let extension = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    let media_type = match extension.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/png",
    };
    Ok(ImageFile {
        media_type: media_type.to_string(),
        data: base64::engine::general_purpose::STANDARD.encode(bytes),
    })
}

const MAX_INDEX_ENTRIES: usize = 3000;
const IGNORED_DIRS: [&str; 8] =
    ["node_modules", "target", ".git", "build", "DerivedData", "dist", ".next", "gen"];

#[tauri::command]
pub async fn list_project_files(root: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let base = std::path::PathBuf::from(&root);
        let mut out = Vec::new();
        walk(&base, &base, &mut out);
        out
    })
    .await
    .map_err(|error| error.to_string())
}

fn walk(base: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
    if out.len() >= MAX_INDEX_ENTRIES {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        if out.len() >= MAX_INDEX_ENTRIES {
            return;
        }
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || IGNORED_DIRS.contains(&name.as_str()) {
            continue;
        }
        if path.is_dir() {
            walk(base, &path, out);
        } else if let Ok(relative) = path.strip_prefix(base) {
            out.push(relative.to_string_lossy().to_string());
        }
    }
}

#[tauri::command]
pub async fn fetch_reference(url: String) -> Result<FetchedReference, String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("only http(s) URLs are supported".to_string());
    }
    let response = reqwest::get(&url).await.map_err(|error| error.to_string())?;
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string();
    let bytes = response.bytes().await.map_err(|error| error.to_string())?;
    let capped = &bytes[..bytes.len().min(MAX_FETCH_BYTES)];
    Ok(FetchedReference {
        content_type,
        body: String::from_utf8_lossy(capped).to_string(),
    })
}
