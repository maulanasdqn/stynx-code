use std::sync::OnceLock;
use std::time::Duration;

static LOGIN_PATH: OnceLock<Option<String>> = OnceLock::new();

fn probe() -> Option<String> {
    if cfg!(windows) {
        return None;
    }
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".into());
    let mut child = std::process::Command::new(shell)
        .args(["-l", "-i", "-c", "printf '__STYNX_PATH__%s' \"$PATH\""])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while child.try_wait().ok()?.is_none() {
        if std::time::Instant::now() > deadline {
            let _ = child.kill();
            return None;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    let output = child.wait_with_output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let path = text.rsplit("__STYNX_PATH__").next()?.trim().to_string();
    (!path.is_empty()).then_some(path)
}

pub fn login_path() -> Option<&'static str> {
    LOGIN_PATH.get_or_init(probe).as_deref()
}

pub fn apply(command: &mut std::process::Command) {
    if let Some(path) = login_path() {
        command.env("PATH", path);
    }
}
