import { bundledLanguages, bundledLanguagesInfo } from "shiki/langs";

const EXTRA = {
  mjs: "javascript", cjs: "javascript", mts: "typescript", cts: "typescript",
  h: "c", hh: "cpp", hpp: "cpp", cc: "cpp", cxx: "cpp", m: "objective-c", mm: "objective-cpp",
  svg: "xml", plist: "xml", xhtml: "html", htm: "html", jsonc: "jsonc", json5: "json5",
  rb: "ruby", kt: "kotlin", kts: "kotlin", ex: "elixir", exs: "elixir", hs: "haskell",
  ml: "ocaml", cs: "csharp", fs: "fsharp", pl: "perl", ps1: "powershell", tf: "terraform",
  gradle: "groovy", clj: "clojure", erl: "erlang", jl: "julia", lock: "toml", fish: "fish",
  env: "dotenv", conf: "ini", cfg: "ini", sql: "sql", txt: "text",
};

const FILES = {
  dockerfile: "docker", containerfile: "docker", makefile: "make", justfile: "just",
  ".env": "dotenv", ".gitignore": "shellscript", ".prettierrc": "json", ".zshrc": "shellscript",
  ".bashrc": "shellscript", "cargo.lock": "toml", "flake.lock": "json",
};

const byKey = new Map();
for (const info of bundledLanguagesInfo) {
  for (const key of [info.id, ...(info.aliases ?? [])]) {
    if (!byKey.has(key)) byKey.set(key, info.id);
  }
}

export const catalog = bundledLanguagesInfo
  .map((info) => ({ id: info.id, name: info.name, aliases: info.aliases ?? [] }))
  .sort((a, b) => a.name.localeCompare(b.name));

export function languageIdFor(path) {
  const file = (path.split("/").pop() ?? path).toLowerCase();
  if (FILES[file] && bundledLanguages[FILES[file]]) return FILES[file];
  if (file.startsWith(".env")) return "dotenv";
  const dot = file.lastIndexOf(".");
  const ext = dot >= 0 ? file.slice(dot + 1) : file;
  const id = EXTRA[ext] ?? byKey.get(ext);
  return id && (bundledLanguages[id] || id === "text") ? id : null;
}

export const loaderFor = (id) => bundledLanguages[id];
