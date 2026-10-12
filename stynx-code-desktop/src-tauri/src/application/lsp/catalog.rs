use std::path::{Path, PathBuf};

pub struct ServerSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub candidates: &'static [(&'static str, &'static [&'static str])],
}

pub const SERVERS: &[ServerSpec] = &[
    ServerSpec { id: "rust", name: "rust-analyzer", extensions: &["rs"], candidates: &[("rust-analyzer", &[])] },
    ServerSpec {
        id: "typescript",
        name: "TypeScript",
        extensions: &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"],
        candidates: &[("vtsls", &["--stdio"]), ("typescript-language-server", &["--stdio"])],
    },
    ServerSpec {
        id: "python",
        name: "Pyright",
        extensions: &["py", "pyi"],
        candidates: &[("basedpyright-langserver", &["--stdio"]), ("pyright-langserver", &["--stdio"])],
    },
    ServerSpec { id: "go", name: "gopls", extensions: &["go"], candidates: &[("gopls", &[])] },
    ServerSpec {
        id: "clangd",
        name: "clangd",
        extensions: &["c", "h", "cc", "cpp", "cxx", "hpp", "hh", "m", "mm"],
        candidates: &[("clangd", &[])],
    },
];

pub fn spec_for(path: &str) -> Option<&'static ServerSpec> {
    let ext = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
    SERVERS.iter().find(|spec| spec.extensions.contains(&ext.as_str()))
}

pub fn language_id(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => "rust",
        "ts" | "mts" | "cts" => "typescript",
        "tsx" => "typescriptreact",
        "js" | "mjs" | "cjs" => "javascript",
        "jsx" => "javascriptreact",
        "py" | "pyi" => "python",
        "go" => "go",
        "c" | "h" => "c",
        "m" => "objective-c",
        "mm" => "objective-cpp",
        _ => "cpp",
    }
}

pub fn find_in_path(program: &str, path_var: Option<&str>) -> Option<PathBuf> {
    let joined = path_var.map(str::to_string).or_else(|| std::env::var("PATH").ok())?;
    let mut dirs: Vec<PathBuf> = std::env::split_paths(&joined).collect();
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        dirs.extend([home.join(".cargo/bin"), home.join(".local/bin"), home.join("go/bin")]);
    }
    if let Ok(user) = std::env::var("USER") {
        dirs.push(PathBuf::from(format!("/etc/profiles/per-user/{user}/bin")));
    }
    dirs.extend(["/run/current-system/sw/bin", "/nix/var/nix/profiles/default/bin", "/opt/homebrew/bin", "/usr/local/bin"].map(PathBuf::from));
    dirs.into_iter().map(|dir| dir.join(program)).find(|candidate| candidate.is_file())
}

pub fn resolve(spec: &ServerSpec, path_var: Option<&str>) -> Option<(PathBuf, &'static [&'static str])> {
    spec.candidates.iter().find_map(|(program, args)| find_in_path(program, path_var).map(|path| (path, *args)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_files_to_servers() {
        assert_eq!(spec_for("src/main.rs").map(|s| s.id), Some("rust"));
        assert_eq!(spec_for("web/App.TSX").map(|s| s.id), Some("typescript"));
        assert_eq!(spec_for("README.md").map(|s| s.id), None);
        assert_eq!(language_id("a/b.tsx"), "typescriptreact");
    }
}
