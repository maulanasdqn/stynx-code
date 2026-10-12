use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
pub struct WorkspaceRoot {
    canonical: PathBuf,
}

impl WorkspaceRoot {
    pub fn new(path: &str) -> Result<Self, String> {
        let root = Path::new(path);
        if !root.is_absolute() {
            return Err("workspace root must be an absolute path".into());
        }
        let canonical = std::fs::canonicalize(root).map_err(|error| format!("workspace root unavailable: {error}"))?;
        Ok(Self { canonical })
    }

    pub fn path(&self) -> &Path {
        &self.canonical
    }

    pub fn resolve(&self, relative: &str) -> Result<PathBuf, String> {
        let joined = safe_join(&self.canonical, relative).ok_or("path escapes workspace root")?;
        let candidate = partial_canonicalize(&joined);
        if !candidate.starts_with(&self.canonical) {
            return Err("path escapes workspace root".into());
        }
        Ok(candidate)
    }

    pub fn relative(&self, absolute: &Path) -> Option<String> {
        let relative = absolute.strip_prefix(&self.canonical).ok()?;
        Some(relative.to_string_lossy().replace('\\', "/"))
    }
}

fn safe_join(root: &Path, relative: &str) -> Option<PathBuf> {
    let relative = Path::new(relative);
    if relative.is_absolute() {
        return None;
    }
    for component in relative.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => return None,
        }
    }
    Some(root.join(relative))
}

fn partial_canonicalize(path: &Path) -> PathBuf {
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    let mut existing = path;
    let mut tail = Vec::new();
    while !existing.exists() {
        match (existing.file_name(), existing.parent()) {
            (Some(name), Some(parent)) => {
                tail.push(name.to_os_string());
                existing = parent;
            }
            _ => break,
        }
    }
    let mut base = existing.canonicalize().unwrap_or_else(|_| existing.to_path_buf());
    for segment in tail.iter().rev() {
        base.push(segment);
    }
    base
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> (tempfile::TempDir, WorkspaceRoot) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let root = WorkspaceRoot::new(dir.path().to_str().unwrap()).unwrap();
        (dir, root)
    }

    #[test]
    fn resolves_inside_and_new_paths() {
        let (_dir, root) = root();
        assert!(root.resolve("src").unwrap().starts_with(root.path()));
        assert!(root.resolve("src/new/file.rs").unwrap().ends_with("src/new/file.rs"));
        assert_eq!(root.resolve("").unwrap(), root.path());
    }

    #[test]
    fn rejects_traversal_and_absolute() {
        let (_dir, root) = root();
        assert!(root.resolve("../outside").is_err());
        assert!(root.resolve("src/../../outside").is_err());
        assert!(root.resolve("/etc/passwd").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_escape() {
        let (dir, root) = root();
        let outside = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(outside.path(), dir.path().join("link")).unwrap();
        assert!(root.resolve("link/secret").is_err());
    }

    #[test]
    fn relative_round_trip() {
        let (_dir, root) = root();
        let absolute = root.resolve("src/main.rs").unwrap();
        assert_eq!(root.relative(&absolute).as_deref(), Some("src/main.rs"));
    }
}
