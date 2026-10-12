use std::io::Write;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::root::WorkspaceRoot;
use crate::domain::ide_fs::{FileContent, FileNode, SaveResult};

pub const READ_CAP: u64 = 5 * 1024 * 1024;
const HIDDEN_DIRS: [&str; 3] = [".git", ".DS_Store", ".hg"];

pub fn fingerprint(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn looks_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(8192)].contains(&0)
}

pub fn list_dir(root: &WorkspaceRoot, relative: &str) -> Result<Vec<FileNode>, String> {
    let dir = root.resolve(relative)?;
    let entries = std::fs::read_dir(&dir).map_err(|error| error.to_string())?;
    let mut nodes: Vec<FileNode> = entries
        .flatten()
        .filter(|entry| !HIDDEN_DIRS.contains(&entry.file_name().to_string_lossy().as_ref()))
        .filter_map(|entry| {
            let path = entry.path();
            Some(FileNode {
                name: entry.file_name().to_string_lossy().to_string(),
                path: root.relative(&path)?,
                is_dir: path.is_dir(),
            })
        })
        .collect();
    nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));
    Ok(nodes)
}

pub fn read(root: &WorkspaceRoot, relative: &str) -> Result<FileContent, String> {
    let path = root.resolve(relative)?;
    let meta = std::fs::metadata(&path).map_err(|error| error.to_string())?;
    let base = FileContent { path: relative.to_string(), ..FileContent::default() };
    if meta.is_dir() {
        return Err("path is a directory".into());
    }
    if meta.len() > READ_CAP {
        return Ok(FileContent { too_large: true, ..base });
    }
    let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
    let fingerprint = fingerprint(&bytes);
    if looks_binary(&bytes) {
        return Ok(FileContent { binary: true, fingerprint, ..base });
    }
    match String::from_utf8(bytes) {
        Ok(text) => Ok(FileContent { content: Some(text), fingerprint, ..base }),
        Err(_) => Ok(FileContent { binary: true, fingerprint, ..base }),
    }
}

pub fn save(root: &WorkspaceRoot, relative: &str, content: &str, expected: &str) -> Result<SaveResult, String> {
    let path = root.resolve(relative)?;
    let current = match std::fs::read(&path) {
        Ok(bytes) => fingerprint(&bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error.to_string()),
    };
    if current != expected {
        return Ok(SaveResult { path: relative.to_string(), fingerprint: current, conflict: true });
    }
    if content.len() as u64 > READ_CAP {
        return Err("file too large to save from the editor (max 5 MiB)".into());
    }
    atomic_write(&path, content.as_bytes())?;
    Ok(SaveResult { path: relative.to_string(), fingerprint: fingerprint(content.as_bytes()), conflict: false })
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or("file has no parent directory")?;
    std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let temp = parent.join(format!(".{name}.stynx-tmp"));
    let mut file = std::fs::File::create(&temp).map_err(|error| error.to_string())?;
    file.write_all(bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    if let Ok(meta) = std::fs::metadata(path) {
        let _ = std::fs::set_permissions(&temp, meta.permissions());
    }
    std::fs::rename(&temp, path).map_err(|error| error.to_string())
}

pub fn create(root: &WorkspaceRoot, relative: &str, is_dir: bool) -> Result<(), String> {
    let path = root.resolve(relative)?;
    if path.exists() {
        return Err("a file or folder with that name already exists".into());
    }
    if is_dir {
        std::fs::create_dir_all(&path).map_err(|error| error.to_string())
    } else {
        atomic_write(&path, b"")
    }
}

pub fn rename(root: &WorkspaceRoot, from: &str, to: &str) -> Result<(), String> {
    let source = root.resolve(from)?;
    let target = root.resolve(to)?;
    if source == root.path() {
        return Err("cannot rename the workspace root".into());
    }
    if target.exists() {
        return Err("a file or folder with that name already exists".into());
    }
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    std::fs::rename(source, target).map_err(|error| error.to_string())
}

pub fn delete(root: &WorkspaceRoot, relative: &str) -> Result<(), String> {
    let path = root.resolve(relative)?;
    if path == root.path() {
        return Err("cannot delete the workspace root".into());
    }
    if path.is_dir() {
        std::fs::remove_dir_all(path).map_err(|error| error.to_string())
    } else {
        std::fs::remove_file(path).map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (tempfile::TempDir, WorkspaceRoot) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), "hello").unwrap();
        let root = WorkspaceRoot::new(dir.path().to_str().unwrap()).unwrap();
        (dir, root)
    }

    #[test]
    fn save_detects_conflict() {
        let (dir, root) = setup();
        let read = read(&root, "a.txt").unwrap();
        std::fs::write(dir.path().join("a.txt"), "changed elsewhere").unwrap();
        let result = save(&root, "a.txt", "mine", &read.fingerprint).unwrap();
        assert!(result.conflict);
        assert_eq!(std::fs::read_to_string(dir.path().join("a.txt")).unwrap(), "changed elsewhere");
    }

    #[test]
    fn save_writes_and_refreshes_fingerprint() {
        let (dir, root) = setup();
        let read = read(&root, "a.txt").unwrap();
        let result = save(&root, "a.txt", "updated", &read.fingerprint).unwrap();
        assert!(!result.conflict);
        assert_eq!(result.fingerprint, fingerprint(b"updated"));
        assert_eq!(std::fs::read_to_string(dir.path().join("a.txt")).unwrap(), "updated");
    }

    #[test]
    fn create_rename_delete_and_list() {
        let (_dir, root) = setup();
        create(&root, "src/lib.rs", false).unwrap();
        create(&root, "docs", true).unwrap();
        rename(&root, "src/lib.rs", "src/main.rs").unwrap();
        let names: Vec<String> = list_dir(&root, "").unwrap().into_iter().map(|n| n.name).collect();
        assert_eq!(names, ["docs", "src", "a.txt"]);
        delete(&root, "docs").unwrap();
        assert!(delete(&root, "").is_err());
    }

    #[test]
    fn binary_files_are_flagged() {
        let (dir, root) = setup();
        std::fs::write(dir.path().join("blob.bin"), [0u8, 1, 2]).unwrap();
        assert!(read(&root, "blob.bin").unwrap().binary);
    }
}
