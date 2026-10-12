import { get } from "svelte/store";
import { ideCreate, ideDelete, ideListDir, ideRename } from "./ide-api.js";
import { expanded, fileIndex, ideMessage, namePrompt, tree } from "./ide-stores.js";
import { confirmDestructive } from "../native.js";

export const parentOf = (path) => (path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "");
export const nameOf = (path) => path.slice(path.lastIndexOf("/") + 1);

export async function loadDir(path) {
  try {
    const nodes = await ideListDir(path);
    tree.update((map) => new Map(map).set(path, nodes));
  } catch (error) {
    ideMessage.set(String(error));
  }
}

export function toggleDir(path) {
  const open = get(expanded);
  const next = new Set(open);
  if (open.has(path)) {
    next.delete(path);
  } else {
    next.add(path);
    if (!get(tree).has(path)) loadDir(path);
  }
  expanded.set(next);
}

export function refreshDirs(paths) {
  const loaded = get(tree);
  const dirs = new Set(paths.map(parentOf));
  for (const dir of dirs) {
    if (loaded.has(dir)) loadDir(dir);
  }
  fileIndex.set([]);
}

export function askName(title, initial = "") {
  return new Promise((resolve) => namePrompt.set({ title, initial, resolve }));
}

async function run(action, dirs) {
  try {
    await action();
    refreshDirs(dirs);
  } catch (error) {
    ideMessage.set(String(error));
  }
}

export async function createEntry(dir, isDir) {
  const name = await askName(isDir ? "New Folder" : "New File");
  if (!name) return null;
  const path = dir ? `${dir}/${name}` : name;
  await run(() => ideCreate(path, isDir), [path]);
  if (dir) expanded.update((set) => new Set(set).add(dir));
  return isDir ? null : path;
}

export async function renameEntry(path) {
  const name = await askName("Rename", nameOf(path));
  if (!name || name === nameOf(path)) return null;
  const parent = parentOf(path);
  const target = parent ? `${parent}/${name}` : name;
  await run(() => ideRename(path, target), [path, target]);
  return target;
}

export async function deleteEntry(path) {
  const ok = await confirmDestructive(`Delete "${nameOf(path)}"?`, "This permanently removes it from disk.", "Delete");
  if (!ok) return false;
  await run(() => ideDelete(path), [path]);
  return true;
}
