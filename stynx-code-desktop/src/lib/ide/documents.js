import { get } from "svelte/store";
import { monaco } from "./monaco-setup.js";
import { languageFor } from "./syntax.js";
import { ideRead, ideSave } from "./ide-api.js";
import { activePath, ideMessage, ideRoot, tabs } from "./ide-stores.js";
import { confirmDestructive } from "../native.js";
import { nameOf } from "./file-tree.js";
import { refreshGit } from "./git.js";
import { attachModel, detachModel, notifySaved, resetLsp } from "./lsp-sync.js";
import { applyMarkers } from "./lsp-providers.js";

const docs = new Map();

const uriFor = (path) => monaco.Uri.file(`${get(ideRoot) ?? ""}/${path}`);

function patchTab(path, patch) {
  tabs.update((list) => list.map((tab) => (tab.path === path ? { ...tab, ...patch } : tab)));
}

function track(path, doc) {
  doc.model.onDidChangeContent(() => {
    const dirty = doc.model.getAlternativeVersionId() !== doc.savedVersion;
    if (dirty !== doc.dirty) {
      doc.dirty = dirty;
      patchTab(path, { dirty });
    }
  });
}

export function modelFor(path) {
  return docs.get(path)?.model ?? null;
}

export async function openFile(path) {
  if (!get(tabs).some((tab) => tab.path === path)) {
    let file;
    try {
      file = await ideRead(path);
    } catch (error) {
      ideMessage.set(String(error));
      return;
    }
    const kind = file.tooLarge ? "large" : file.binary ? "binary" : "text";
    if (kind === "text" && !docs.has(path)) {
      const model = monaco.editor.createModel(file.content ?? "", await languageFor(path), uriFor(path));
      const doc = { model, fingerprint: file.fingerprint, savedVersion: model.getAlternativeVersionId(), dirty: false };
      track(path, doc);
      docs.set(path, doc);
      attachModel(path, model);
      applyMarkers(path, model);
    }
    tabs.update((list) => [...list, { path, name: nameOf(path), kind, dirty: false, conflict: false }]);
  }
  activePath.set(path);
}

function markSaved(path, fingerprint) {
  const doc = docs.get(path);
  doc.fingerprint = fingerprint;
  doc.savedVersion = doc.model.getAlternativeVersionId();
  doc.dirty = false;
  patchTab(path, { dirty: false, conflict: false });
}

export async function saveFile(path = get(activePath)) {
  const doc = path && docs.get(path);
  if (!doc) return;
  try {
    const result = await ideSave(path, doc.model.getValue(), doc.fingerprint);
    if (result.conflict) {
      doc.diskFingerprint = result.fingerprint;
      patchTab(path, { conflict: true });
      return;
    }
    markSaved(path, result.fingerprint);
    ideMessage.set(`Saved ${nameOf(path)}`);
    notifySaved(path);
    refreshGit(200);
  } catch (error) {
    ideMessage.set(String(error));
  }
}

function replaceContent(doc, content) {
  doc.model.pushEditOperations([], [{ range: doc.model.getFullModelRange(), text: content }], () => null);
}

export async function reloadFromDisk(path) {
  const doc = docs.get(path);
  if (!doc) return;
  let file;
  try {
    file = await ideRead(path);
  } catch {
    patchTab(path, { conflict: true });
    return;
  }
  if (file.fingerprint === doc.fingerprint || file.content === null) return;
  if (doc.dirty) {
    doc.diskFingerprint = file.fingerprint;
    patchTab(path, { conflict: true });
    return;
  }
  replaceContent(doc, file.content);
  markSaved(path, file.fingerprint);
}

export async function resolveConflict(path, choice) {
  const doc = docs.get(path);
  if (!doc) return;
  if (choice === "mine") {
    doc.fingerprint = doc.diskFingerprint ?? doc.fingerprint;
    patchTab(path, { conflict: false });
    await saveFile(path);
    return;
  }
  const file = await ideRead(path);
  replaceContent(doc, file.content ?? "");
  markSaved(path, file.fingerprint);
}

export async function closeTab(path) {
  const doc = docs.get(path);
  if (doc?.dirty) {
    const ok = await confirmDestructive(`Close "${nameOf(path)}"?`, "Your unsaved changes will be lost.", "Discard");
    if (!ok) return;
  }
  if (doc) detachModel(path);
  doc?.model.dispose();
  docs.delete(path);
  const list = get(tabs);
  const index = list.findIndex((tab) => tab.path === path);
  const next = list.filter((tab) => tab.path !== path);
  tabs.set(next);
  if (get(activePath) === path) activePath.set(next[Math.min(index, next.length - 1)]?.path ?? null);
}

export async function retargetTab(from, to) {
  const doc = docs.get(from);
  if (!doc) return;
  const model = monaco.editor.createModel(doc.model.getValue(), await languageFor(to), uriFor(to));
  detachModel(from);
  doc.model.dispose();
  docs.delete(from);
  const moved = { ...doc, model };
  track(to, moved);
  docs.set(to, moved);
  attachModel(to, model);
  tabs.update((list) => list.map((tab) => (tab.path === from ? { ...tab, path: to, name: nameOf(to) } : tab)));
  if (get(activePath) === from) activePath.set(to);
}

export function reattachLanguageServers() {
  for (const [path, doc] of docs) {
    detachModel(path);
    attachModel(path, doc.model);
  }
}

export function resetDocuments() {
  resetLsp();
  for (const doc of docs.values()) doc.model.dispose();
  docs.clear();
  tabs.set([]);
  activePath.set(null);
}
