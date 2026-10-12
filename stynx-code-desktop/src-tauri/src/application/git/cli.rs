use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::Mutex;

static GIT_LOCK: Mutex<()> = Mutex::new(());

pub fn git(dir: &Path, args: &[&str]) -> Result<Output, String> {
    let _guard = GIT_LOCK.lock().map_err(|e| e.to_string())?;
    let mut command = std::process::Command::new("git");
    crate::application::env_path::apply(&mut command);
    command
        .args(args)
        .current_dir(dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|error| format!("failed to run git: {error}"))
}

pub fn failure(output: &Output, fallback: &str) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    [stderr, stdout].into_iter().find(|text| !text.is_empty()).unwrap_or_else(|| fallback.to_string())
}

pub fn run(dir: &Path, args: &[&str], fallback: &str) -> Result<Output, String> {
    let output = git(dir, args)?;
    if output.status.success() { Ok(output) } else { Err(failure(&output, fallback)) }
}

pub fn repo_root(workspace: &Path) -> Option<PathBuf> {
    let output = git(workspace, &["rev-parse", "--show-toplevel"]).ok()?;
    if !output.status.success() {
        return None;
    }
    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    std::fs::canonicalize(root).ok()
}

pub fn has_head(root: &Path) -> bool {
    git(root, &["rev-parse", "--verify", "-q", "HEAD"]).is_ok_and(|output| output.status.success())
}
