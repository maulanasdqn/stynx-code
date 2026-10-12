use std::path::Path;

use super::cli::{git, repo_root};
use crate::application::workspace::root::WorkspaceRoot;
use crate::domain::ide_git::GitDiff;

const DIFF_CAP: usize = 2 * 1024 * 1024;

fn looks_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(8192)].contains(&0)
}

pub fn diff(workspace: &Path, path: &str, staged: bool) -> Result<GitDiff, String> {
    let root = repo_root(workspace).ok_or("not a git repository")?;
    let absolute = WorkspaceRoot::new(&root.to_string_lossy())?.resolve(path)?;
    let show = |spec: String| -> Vec<u8> {
        git(&root, &["show", &spec]).ok().filter(|o| o.status.success()).map(|o| o.stdout).unwrap_or_default()
    };
    let (original, modified) = if staged {
        (show(format!("HEAD:{path}")), show(format!(":{path}")))
    } else {
        let index = show(format!(":{path}"));
        let base = if index.is_empty() { show(format!("HEAD:{path}")) } else { index };
        (base, std::fs::read(&absolute).unwrap_or_default())
    };
    let mut result = GitDiff { path: path.to_string(), staged, ..GitDiff::default() };
    if original.len() > DIFF_CAP || modified.len() > DIFF_CAP {
        result.too_large = true;
    } else if looks_binary(&original) || looks_binary(&modified) {
        result.binary = true;
    } else {
        result.original = String::from_utf8_lossy(&original).into_owned();
        result.modified = String::from_utf8_lossy(&modified).into_owned();
    }
    Ok(result)
}
