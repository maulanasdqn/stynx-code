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
    let env = format!(
        "STYNX_ZDOTDIR=\"{dir}\"\n[[ -f \"$HOME/.zshenv\" ]] && source \"$HOME/.zshenv\"\nif [[ -n \"$ZDOTDIR\" && \"$ZDOTDIR\" != \"$STYNX_ZDOTDIR\" ]]; then STYNX_USER_ZDOTDIR=\"$ZDOTDIR\"; else STYNX_USER_ZDOTDIR=\"$HOME\"; fi\nZDOTDIR=\"$STYNX_ZDOTDIR\"\n[[ \"$STYNX_USER_ZDOTDIR\" != \"$HOME\" && -f \"$STYNX_USER_ZDOTDIR/.zshenv\" ]] && source \"$STYNX_USER_ZDOTDIR/.zshenv\"\nZDOTDIR=\"$STYNX_ZDOTDIR\"\n",
        dir = dir.display()
    );
    let user_file = |name: &str| format!("[[ -f \"$STYNX_USER_ZDOTDIR/{name}\" ]] && source \"$STYNX_USER_ZDOTDIR/{name}\"\n");
    let profile = user_file(".zprofile");
    let login = user_file(".zlogin");
    let rc = format!(
        "ZDOTDIR=\"$STYNX_USER_ZDOTDIR\"\n{}bindkey '^?' backward-delete-char\nbindkey '^H' backward-delete-char\nbindkey '^[[3~' delete-char\n",
        user_file(".zshrc")
    );
    std::fs::write(dir.join(".zshenv"), env).ok()?;
    std::fs::write(dir.join(".zprofile"), profile).ok()?;
    std::fs::write(dir.join(".zshrc"), rc).ok()?;
    std::fs::write(dir.join(".zlogin"), login).ok()?;
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

    #[cfg(target_os = "macos")]
    #[test]
    fn zsh_wrapper_loads_user_env_and_rc() {
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join(".zshenv"), "export ZSH=/tmp/ohmy\n").unwrap();
        std::fs::write(home.path().join(".zshrc"), "echo \"rc:$ZSH\"\n").unwrap();
        let dir = zsh_keys_dir("/bin/zsh", 9001).unwrap();
        let out = std::process::Command::new("/bin/zsh")
            .args(["-l", "-i", "-c", "true"])
            .env("HOME", home.path())
            .env("ZDOTDIR", &dir)
            .output()
            .unwrap();
        let _ = std::fs::remove_dir_all(dir);
        assert!(String::from_utf8_lossy(&out.stdout).contains("rc:/tmp/ohmy"), "{:?}", out);
    }
}

#[cfg(all(test, target_os = "macos"))]
mod real_home {
    #[test]
    #[ignore = "uses the developer's real zsh config"]
    fn real_zsh_config_loads_cleanly() {
        let dir = super::zsh_keys_dir("/bin/zsh", 9002).unwrap();
        let out = std::process::Command::new("/bin/zsh")
            .args(["-l", "-i", "-c", "echo ZSH=$ZSH"])
            .env("ZDOTDIR", &dir)
            .output()
            .unwrap();
        let _ = std::fs::remove_dir_all(dir);
        let stderr = String::from_utf8_lossy(&out.stderr);
        println!("stdout: {}\nstderr: {stderr}", String::from_utf8_lossy(&out.stdout));
        assert!(!stderr.contains("no such file or directory"), "{stderr}");
    }
}
