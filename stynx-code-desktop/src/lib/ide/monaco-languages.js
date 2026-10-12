const EXT_LANG = {
  ts: "typescript", tsx: "typescript", mts: "typescript", cts: "typescript",
  js: "javascript", jsx: "javascript", mjs: "javascript", cjs: "javascript",
  json: "javascript", jsonc: "javascript", svelte: "html", vue: "html",
  css: "css", scss: "scss", sass: "scss", less: "less",
  html: "html", htm: "html", xhtml: "html", xml: "xml", svg: "xml", plist: "xml",
  md: "markdown", markdown: "markdown", mdx: "markdown",
  rs: "rust", py: "python", pyi: "python", go: "go", swift: "swift",
  sh: "shell", bash: "shell", zsh: "shell", nix: "shell",
  yaml: "yaml", yml: "yaml", sql: "sql",
  c: "cpp", h: "cpp", hpp: "cpp", hh: "cpp", cpp: "cpp", cc: "cpp", cxx: "cpp", m: "cpp", mm: "cpp",
  java: "java", php: "php",
  ini: "ini", toml: "ini", cfg: "ini", conf: "ini", lock: "ini",
};

const FILE_LANG = { Dockerfile: "shell", Makefile: "shell", justfile: "shell" };

export function langFromPath(path) {
  const file = path.split("/").pop() ?? path;
  if (FILE_LANG[file]) return FILE_LANG[file];
  const dot = file.lastIndexOf(".");
  const ext = dot > 0 ? file.slice(dot + 1).toLowerCase() : "";
  return EXT_LANG[ext] ?? "plaintext";
}
