use std::path::{Path, PathBuf};

pub fn shell_args(shell: &str) -> &'static [&'static str] {
    let name = shell.rsplit(['/', '\\']).next().unwrap_or(shell).to_ascii_lowercase();
    match name.trim_end_matches(".exe") {
        "cmd" => &[],
        "powershell" | "pwsh" => &["-NoLogo", "-NoExit"],
        "nu" => &["--login", "--interactive"],
        "bash" | "rbash" | "zsh" | "fish" | "sh" | "dash" | "ksh" => &["-l", "-i"],
        _ => &[],
    }
}

pub fn default_shell() -> String {
    if cfg!(windows) {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".into())
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into())
    }
}

fn executable(path: &Path) -> bool {
    let Ok(meta) = path.metadata() else { return false };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        meta.is_file() && meta.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        meta.is_file()
    }
}

pub fn available_shells() -> Vec<String> {
    let mut candidates = vec![PathBuf::from(default_shell())];
    if let Ok(listed) = std::fs::read_to_string("/etc/shells") {
        candidates.extend(listed.lines().map(str::trim).filter(|l| l.starts_with('/')).map(PathBuf::from));
    }
    let mut seen = std::collections::HashSet::new();
    candidates
        .into_iter()
        .filter(|path| executable(path))
        .filter(|path| seen.insert(std::fs::canonicalize(path).unwrap_or_else(|_| path.clone())))
        .map(|path| path.to_string_lossy().to_string())
        .collect()
}

pub fn zsh_keys_dir(shell: &str, id: u32) -> Option<PathBuf> {
    if !cfg!(target_os = "macos") || !shell.ends_with("zsh") {
        return None;
    }
    let dir = std::env::temp_dir().join(format!("stynx-zdot-{}-{id}", std::process::id()));
    std::fs::create_dir_all(&dir).ok()?;
    let profile = "[[ -f \"$HOME/.zprofile\" ]] && source \"$HOME/.zprofile\"\n[[ -f \"$HOME/.zlogin\" ]] && source \"$HOME/.zlogin\"\n";
    let rc = "[[ -f \"$HOME/.zshrc\" ]] && source \"$HOME/.zshrc\"\nbindkey '^?' backward-delete-char\nbindkey '^H' backward-delete-char\nbindkey '^[[3~' delete-char\n";
    std::fs::write(dir.join(".zprofile"), profile).ok()?;
    std::fs::write(dir.join(".zshrc"), rc).ok()?;
    Some(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_flags_per_shell() {
        for shell in ["/bin/zsh", "/bin/bash", "/usr/bin/fish"] {
            assert_eq!(shell_args(shell), ["-l", "-i"]);
        }
        assert_eq!(shell_args("C:\\PowerShell\\pwsh.exe"), ["-NoLogo", "-NoExit"]);
        assert!(shell_args("cmd.exe").is_empty());
    }
}
