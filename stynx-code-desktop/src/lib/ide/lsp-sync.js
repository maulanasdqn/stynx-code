import { get } from "svelte/store";
import { lspChange, lspClose, lspOpen, lspSave } from "./ide-api.js";
import { ideRoot } from "./ide-stores.js";

const tracked = new Map();
const DEBOUNCE = 150;

export function pathOf(model) {
  const root = get(ideRoot);
  const full = model?.uri?.path ?? "";
  return root && full.startsWith(`${root}/`) ? full.slice(root.length + 1) : null;
}

async function flushOne(path) {
  const entry = tracked.get(path);
  if (!entry?.pending) return;
  clearTimeout(entry.timer);
  entry.pending = false;
  await lspChange(path, entry.model.getValue(), entry.model.getVersionId()).catch(() => {});
}

export async function attachModel(path, model) {
  const active = await lspOpen(path, model.getValue()).catch(() => false);
  if (!active) return;
  const entry = { model, pending: false, timer: null };
  entry.subscription = model.onDidChangeContent(() => {
    entry.pending = true;
    clearTimeout(entry.timer);
    entry.timer = setTimeout(() => flushOne(path), DEBOUNCE);
  });
  tracked.set(path, entry);
}

export const flush = (path) => flushOne(path);

export function notifySaved(path) {
  if (tracked.has(path)) flushOne(path).then(() => lspSave(path).catch(() => {}));
}

export function detachModel(path) {
  const entry = tracked.get(path);
  if (!entry) return;
  clearTimeout(entry.timer);
  entry.subscription.dispose();
  tracked.delete(path);
  lspClose(path).catch(() => {});
}

export function resetLsp() {
  for (const path of [...tracked.keys()]) detachModel(path);
}
