use std::path::{Path, PathBuf};

pub struct ServerSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub candidates: &'static [(&'static str, &'static [&'static str])],
    pub npm: &'static [&'static str],
    pub hint: &'static str,
    pub needs_tsdk: bool,
}

const fn server(
    id: &'static str,
    name: &'static str,
    extensions: &'static [&'static str],
    candidates: &'static [(&'static str, &'static [&'static str])],
    npm: &'static [&'static str],
    hint: &'static str,
) -> ServerSpec {
    ServerSpec { id, name, extensions, candidates, npm, hint, needs_tsdk: false }
}

const STDIO: &[&str] = &["--stdio"];

pub const SERVERS: &[ServerSpec] = &[
    server("rust", "rust-analyzer", &["rs"], &[("rust-analyzer", &[])], &[], "rustup component add rust-analyzer"),
    server(
        "typescript",
        "TypeScript",
        &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"],
        &[("vtsls", STDIO), ("typescript-language-server", STDIO)],
        &["@vtsls/language-server", "typescript"],
        "",
    ),
    ServerSpec {
        needs_tsdk: true,
        ..server("astro", "Astro", &["astro"], &[("astro-ls", STDIO)], &["@astrojs/language-server", "typescript"], "")
    },
    server("svelte", "Svelte", &["svelte"], &[("svelteserver", STDIO)], &["svelte-language-server", "typescript"], ""),
    server("html", "HTML", &["html", "htm"], &[("vscode-html-language-server", STDIO)], &["vscode-langservers-extracted"], ""),
    server("css", "CSS", &["css", "scss", "less"], &[("vscode-css-language-server", STDIO)], &["vscode-langservers-extracted"], ""),
    server("json", "JSON", &["json", "jsonc"], &[("vscode-json-language-server", STDIO)], &["vscode-langservers-extracted"], ""),
    server("yaml", "YAML", &["yaml", "yml"], &[("yaml-language-server", STDIO)], &["yaml-language-server"], ""),
    server("bash", "Bash", &["sh", "bash", "zsh"], &[("bash-language-server", &["start"])], &["bash-language-server"], ""),
    server("docker", "Dockerfile", &["dockerfile"], &[("docker-langserver", STDIO)], &["dockerfile-language-server-nodejs"], ""),
    server("toml", "TOML", &["toml"], &[("taplo", &["lsp", "stdio"])], &["@taplo/cli"], ""),
    server(
        "python",
        "Pyright",
        &["py", "pyi"],
        &[("basedpyright-langserver", STDIO), ("pyright-langserver", STDIO)],
        &["basedpyright"],
        "",
    ),
    server("go", "gopls", &["go"], &[("gopls", &[])], &[], "go install golang.org/x/tools/gopls@latest"),
    server(
        "clangd",
        "clangd",
        &["c", "h", "cc", "cpp", "cxx", "hpp", "hh", "m", "mm"],
        &[("clangd", &[])],
        &[],
        "xcode-select --install (macOS) or install clangd",
    ),
];

fn key(path: &str) -> String {
    let file = Path::new(path).file_name().and_then(|f| f.to_str()).unwrap_or(path).to_ascii_lowercase();
    if file == "dockerfile" || file.starts_with("dockerfile.") {
        return "dockerfile".into();
    }
    Path::new(&file).extension().and_then(|e| e.to_str()).unwrap_or("").to_string()
}

pub fn spec_for(path: &str) -> Option<&'static ServerSpec> {
    let ext = key(path);
    SERVERS.iter().find(|spec| spec.extensions.contains(&ext.as_str()))
}

pub fn spec_by_id(id: &str) -> Option<&'static ServerSpec> {
    SERVERS.iter().find(|spec| spec.id == id)
}

pub fn language_id(path: &str) -> &'static str {
    match key(path).as_str() {
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
        "astro" => "astro",
        "svelte" => "svelte",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" => "scss",
        "less" => "less",
        "json" => "json",
        "jsonc" => "jsonc",
        "yaml" | "yml" => "yaml",
        "sh" | "bash" | "zsh" => "shellscript",
        "dockerfile" => "dockerfile",
        "toml" => "toml",
        _ => "cpp",
    }
}

pub fn install_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".stynx").join("lsp"))
}

pub fn find_in_path(program: &str, path_var: Option<&str>) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = install_dir().map(|dir| dir.join("node_modules/.bin")).into_iter().collect();
    let joined = path_var.map(str::to_string).or_else(|| std::env::var("PATH").ok()).unwrap_or_default();
    dirs.extend(std::env::split_paths(&joined));
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

pub fn typescript_sdk(root: &Path) -> Option<PathBuf> {
    let local = root.join("node_modules/typescript/lib");
    if local.is_dir() {
        return Some(local);
    }
    install_dir().map(|dir| dir.join("node_modules/typescript/lib")).filter(|dir| dir.is_dir())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_files_to_servers() {
        assert_eq!(spec_for("src/main.rs").map(|s| s.id), Some("rust"));
        assert_eq!(spec_for("web/App.TSX").map(|s| s.id), Some("typescript"));
        assert_eq!(spec_for("src/claim-cell.astro").map(|s| s.id), Some("astro"));
        assert_eq!(spec_for("Dockerfile").map(|s| s.id), Some("docker"));
        assert_eq!(spec_for("README.md").map(|s| s.id), None);
        assert_eq!(language_id("a/b.tsx"), "typescriptreact");
        assert_eq!(language_id("x.astro"), "astro");
    }
}
