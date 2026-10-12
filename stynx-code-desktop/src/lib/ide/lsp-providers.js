import { get } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { monaco } from "./monaco-setup.js";
import { lspCompletion, lspDefinition, lspHover, lspResolve } from "./ide-api.js";
import { diagnostics, ideRoot, installLogs, lspStatus, revealTarget } from "./ide-stores.js";
import { flush, pathOf } from "./lsp-sync.js";
import { toCompletion, toHoverContents, toMarkers, toRange } from "./lsp-convert.js";

const TRIGGERS = [".", ":", "<", '"', "'", "/", "@", ">", "(", ",", "$"];

let installed = false;
let openFileHandler = null;

const uriFor = (path) => monaco.Uri.file(`${get(ideRoot)}/${path}`);

export function applyMarkers(path, model) {
  const list = get(diagnostics).get(path);
  if (model && list) monaco.editor.setModelMarkers(model, "lsp", toMarkers(list));
}

async function provideCompletionItems(model, position) {
  const path = pathOf(model);
  if (!path) return { suggestions: [] };
  await flush(path);
  const result = await lspCompletion(path, position.lineNumber - 1, position.column - 1).catch(() => null);
  const items = Array.isArray(result) ? result : result?.items ?? [];
  const word = model.getWordUntilPosition(position);
  const fallback = new monaco.Range(position.lineNumber, word.startColumn, position.lineNumber, word.endColumn);
  const suggestions = items.map((item) => ({ ...toCompletion(item, fallback), lspPath: path }));
  return { suggestions, incomplete: Boolean(result?.isIncomplete) };
}

async function resolveCompletionItem(item) {
  if (!item.lspPath || !item.lspItem) return item;
  const resolved = await lspResolve(item.lspPath, item.lspItem).catch(() => null);
  if (!resolved) return item;
  return { ...item, ...toCompletion(resolved, item.range), range: item.range, lspPath: item.lspPath };
}

async function provideHover(model, position) {
  const path = pathOf(model);
  if (!path) return null;
  await flush(path);
  const hover = await lspHover(path, position.lineNumber - 1, position.column - 1).catch(() => null);
  if (!hover?.contents) return null;
  return { contents: toHoverContents(hover.contents), range: hover.range ? toRange(hover.range) : undefined };
}

async function provideDefinition(model, position) {
  const path = pathOf(model);
  if (!path) return [];
  await flush(path);
  const locations = await lspDefinition(path, position.lineNumber - 1, position.column - 1).catch(() => []);
  return locations.map((loc) => ({ uri: uriFor(loc.path), range: toRange(loc.range) }));
}

function openCodeEditor(_source, resource, selection) {
  const root = get(ideRoot);
  if (!root || !resource.path.startsWith(`${root}/`) || !openFileHandler) return false;
  const path = resource.path.slice(root.length + 1);
  const line = selection?.startLineNumber ?? selection?.lineNumber ?? 1;
  const column = selection?.startColumn ?? selection?.column ?? 1;
  openFileHandler(path).then(() => revealTarget.set({ path, line, column }));
  return true;
}

export async function installLsp(openFile) {
  openFileHandler = openFile;
  if (installed) return;
  installed = true;
  monaco.languages.registerCompletionItemProvider("*", { triggerCharacters: TRIGGERS, provideCompletionItems, resolveCompletionItem });
  monaco.languages.registerHoverProvider("*", { provideHover });
  monaco.languages.registerDefinitionProvider("*", { provideDefinition });
  monaco.editor.registerEditorOpener({ openCodeEditor });
  await listen("ide-lsp", ({ payload }) => {
    if (payload.type === "diagnostics") {
      diagnostics.update((map) => {
        const next = new Map(map);
        if (payload.diagnostics.length) next.set(payload.path, payload.diagnostics);
        else next.delete(payload.path);
        return next;
      });
      const model = monaco.editor.getModel(uriFor(payload.path));
      if (model) monaco.editor.setModelMarkers(model, "lsp", toMarkers(payload.diagnostics));
    } else if (payload.type === "installLog") {
      installLogs.update((map) => new Map(map).set(payload.id, [...(map.get(payload.id) ?? []), payload.line].slice(-40)));
    } else if (payload.type === "status") {
      lspStatus.update((map) => new Map(map).set(payload.server, { state: payload.state, message: payload.message }));
    }
  });
}
