use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use super::catalog::{ServerSpec, install_dir};

pub async fn npm<F>(spec: &ServerSpec, uninstall: bool, path_var: Option<&str>, on_line: F) -> Result<(), String>
where
    F: Fn(String) + Send + Sync + Clone + 'static,
{
    if spec.npm.is_empty() {
        return Err(format!("{} isn't installable from Stynx — run: {}", spec.name, spec.hint));
    }
    let dir = install_dir().ok_or("cannot determine home directory")?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    if !dir.join("package.json").exists() {
        std::fs::write(dir.join("package.json"), "{\n  \"name\": \"stynx-language-servers\",\n  \"private\": true\n}\n").map_err(|e| e.to_string())?;
    }
    let packages: &[&str] = if uninstall { &spec.npm[..1] } else { spec.npm };
    let mut command = Command::new("npm");
    command
        .arg(if uninstall { "uninstall" } else { "install" })
        .args(["--prefix", &dir.to_string_lossy(), "--no-audit", "--no-fund", "--loglevel", "error"])
        .args(packages)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    if let Some(path) = path_var {
        command.env("PATH", path);
    }
    let mut child = command.spawn().map_err(|e| format!("npm not found ({e}) — install Node.js first"))?;
    for stream in [child.stdout.take().map(|s| Box::new(s) as Box<dyn tokio::io::AsyncRead + Unpin + Send>), child.stderr.take().map(|s| Box::new(s) as _)]
        .into_iter()
        .flatten()
    {
        let sink = on_line.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stream).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if !line.trim().is_empty() {
                    sink(line);
                }
            }
        });
    }
    let status = child.wait().await.map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err(format!("npm exited with {status}")) }
}
