# Stynx Desktop (cross-platform)

Tauri v2 + Svelte front-end for the stynx engine, targeting macOS, Windows,
and Linux. The Rust backend (`src-tauri/`) links `stynx-code-app` directly —
no uniffi layer — and exposes the same command surface the macOS SwiftUI app
(`stynx-code-mac/`) consumes over FFI. Engine events stream to the webview as
a single `engine-event` Tauri event, tagged by `type`.

## Layout

- `src-tauri/src/` — Rust backend (workspace member `stynx-code-desktop`),
  layered like the rest of the workspace:
  - `domain/` — UI event enum and serializable models; no Tauri types.
  - `application/` — session state and the engine dispatch loop; takes an
    event-sink closure, so it never touches Tauri directly.
  - `infrastructure/` — the Tauri adapters: event emitter, engine bridge
    drains, and the `#[tauri::command]` surface (session, messaging,
    settings, history).
- `src/` — Svelte UI: `lib/` holds the logic (invoke wrappers, engine-event
  reducer, messaging queue, session actions, markdown parser), `components/`
  holds presentation only; all files kebab-case and under 200 lines.

## Code mode (IDE)

Switch between Chat and Code with the toolbar control or ⌘⇧E. Code mode is a
lightweight IDE on the agent's workspace, ported from
[Koma](https://github.com/aula-id/koma) (see `THIRD_PARTY_NOTICES.md`):

- Explorer, Monaco editor tabs, quick open (⌘P), save (⌘S) guarded against
  on-disk changes; agent edits refresh open files.
- Search across files (⌘⇧F) with case and regex options.
- Integrated terminal (⌘\`) backed by a real PTY.
- Source Control: stage, unstage, discard, commit, side-by-side diffs.
- Syntax highlighting for 240+ languages (TextMate grammars), installed on
  demand from Extensions → Languages.
- Language servers: completions, hover, go-to-definition (F12), format
  document, outline (⌘⇧O), parameter hints and diagnostics. npm-based servers
  install into `~/.stynx/lsp` from Extensions → LSP.
- MCP marketplace: search the official registry and install servers into
  `.mcp.json` or `~/.claude/mcp.json` from Extensions → MCP.
- Interns: add, edit and remove delegate models and their API keys.

Monaco loads lazily the first time Code mode opens, so Chat stays light.

## Develop

```bash
cd stynx-code-desktop
npm install
npm run tauri dev
```

## Build installers

```bash
npm run tauri build   # .dmg / .msi + .exe / .deb + .AppImage per host OS
```

Regenerate the full icon set from the shared logo before shipping:

```bash
npm run tauri icon src-tauri/icons/icon.png
```

## Platform notes

- Linux needs WebKitGTK (`libwebkit2gtk-4.1-dev` and friends at build time).
- Windows needs the WebView2 runtime (preinstalled on Windows 11).
- The engine's bash tool assumes a POSIX shell; on Windows run under WSL or
  Git Bash until a PowerShell adapter lands in `stynx-code-tools`.
