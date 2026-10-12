# Third-party notices

## Koma

Portions of the Stynx desktop IDE (Code mode) are derived from **Koma**,
https://github.com/aula-id/koma, Copyright the Koma authors (aula-id),
licensed under the Apache License, Version 2.0. The full license text is in
[`licenses/koma-APACHE-2.0.txt`](licenses/koma-APACHE-2.0.txt).

These files were modified by Stynx: they were translated from TypeScript/React
and Koma's Rust host into Svelte and Tauri, renamed, restructured, and adapted
to the Stynx design system.

| Stynx file | Derived from (Koma) |
|---|---|
| `src-tauri/src/application/workspace/root.rs` | `src-agent/src/app/runtime/client/file_ops.rs` (`resolve_contained`, `safe_join`, `partial_canonicalize`) |
| `src-tauri/src/application/workspace/fs_ops.rs` | `src-agent/src/app/runtime/client/file_ops.rs` (read/save with fingerprints, binary sniff) |
| `src/lib/ide/monaco-setup.js` | `src-webgui/src/lib/monaco-setup.ts` (editor core, inlined worker, language contributions) |
| `src/lib/ide/monaco-languages.js` | `src-webgui/src/lib/monaco-setup.ts` (`EXT_LANG`, `langFromPath`) |
| `src/lib/ide/color.js` | `src-webgui/src/lib/monaco-setup.ts` (`cssColorToHex`, `resolveVarHex`, `mixHex`) |
| `src/lib/ide/monaco-theme.js` | `src-webgui/src/lib/monaco-setup.ts` (`applyKomaTheme` color mapping) |

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
these files except in compliance with the License. Unless required by
applicable law or agreed to in writing, software distributed under the License
is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.
