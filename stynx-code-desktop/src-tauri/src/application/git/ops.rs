use std::path::Path;

use super::cli::{git, has_head, repo_root, run};
use crate::application::workspace::root::WorkspaceRoot;

fn repo(workspace: &Path) -> Result<std::path::PathBuf, String> {
    repo_root(workspace).ok_or_else(|| "not a git repository".to_string())
}

fn with_paths<'a>(base: &[&'a str], paths: &'a [String]) -> Vec<&'a str> {
    let mut args = base.to_vec();
    args.push("--");
    args.extend(paths.iter().map(String::as_str));
    args
}

pub fn stage(workspace: &Path, paths: &[String]) -> Result<(), String> {
    let root = repo(workspace)?;
    run(&root, &with_paths(&["add", "-A"], paths), "git add failed").map(|_| ())
}

pub fn unstage(workspace: &Path, paths: &[String]) -> Result<(), String> {
    let root = repo(workspace)?;
    let base: &[&str] = if has_head(&root) { &["restore", "--staged"] } else { &["rm", "--cached", "-r", "-q"] };
    run(&root, &with_paths(base, paths), "git unstage failed").map(|_| ())
}

pub fn discard(workspace: &Path, paths: &[String]) -> Result<(), String> {
    let root = repo(workspace)?;
    let sandbox = WorkspaceRoot::new(&root.to_string_lossy())?;
    let mut tracked = Vec::new();
    for path in paths {
        let known = git(&root, &["ls-files", "--error-unmatch", "--", path]).is_ok_and(|o| o.status.success());
        if known {
            tracked.push(path.clone());
        } else {
            let absolute = sandbox.resolve(path)?;
            std::fs::remove_file(absolute).map_err(|e| format!("{path}: {e}"))?;
        }
    }
    if !tracked.is_empty() {
        run(&root, &with_paths(&["restore"], &tracked), "git restore failed")?;
    }
    Ok(())
}

pub fn commit(workspace: &Path, message: &str) -> Result<(), String> {
    if message.trim().is_empty() {
        return Err("commit message is empty".into());
    }
    let root = repo(workspace)?;
    run(&root, &["commit", "-m", message], "git commit failed").map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::git::{diff::diff, status::status};

    fn repo_dir() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for args in [vec!["init", "-q", "-b", "main"], vec!["config", "user.email", "t@t"], vec!["config", "user.name", "t"]] {
            assert!(git(dir.path(), &args).unwrap().status.success());
        }
        dir
    }

    #[test]
    fn stage_unstage_commit_and_discard_round_trip() {
        let dir = repo_dir();
        let path = std::fs::canonicalize(dir.path()).unwrap();
        std::fs::write(path.join("a.txt"), "one\n").unwrap();
        stage(&path, &["a.txt".into()]).unwrap();
        assert_eq!(status(&path).unwrap().staged[0].status, "A");
        unstage(&path, &["a.txt".into()]).unwrap();
        assert_eq!(status(&path).unwrap().unstaged[0].status, "?");
        stage(&path, &["a.txt".into()]).unwrap();
        commit(&path, "first").unwrap();
        assert!(commit(&path, "  ").is_err());

        std::fs::write(path.join("a.txt"), "two\n").unwrap();
        let changes = diff(&path, "a.txt", false).unwrap();
        assert_eq!((changes.original.as_str(), changes.modified.as_str()), ("one\n", "two\n"));
        std::fs::write(path.join("scratch.txt"), "x").unwrap();
        discard(&path, &["a.txt".into(), "scratch.txt".into()]).unwrap();
        assert_eq!(std::fs::read_to_string(path.join("a.txt")).unwrap(), "one\n");
        assert!(!path.join("scratch.txt").exists());
        assert!(status(&path).unwrap().unstaged.is_empty());
    }

    #[test]
    fn discard_rejects_escaping_paths() {
        let dir = repo_dir();
        let path = std::fs::canonicalize(dir.path()).unwrap();
        assert!(discard(&path, &["../outside.txt".into()]).is_err());
    }
}
