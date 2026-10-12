import { get } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { ideAttach, ideIndex } from "./ide-api.js";
import { expanded, fileIndex, ideMessage, ideRoot, registerAgentEditHook, tree } from "./ide-stores.js";
import { loadDir, refreshDirs } from "./file-tree.js";
import { reloadFromDisk, resetDocuments } from "./documents.js";

let unlisten = null;

export async function attachWorkspace() {
  try {
    const root = await ideAttach();
    resetDocuments();
    tree.set(new Map());
    expanded.set(new Set([""]));
    fileIndex.set([]);
    ideRoot.set(root);
    ideMessage.set("");
    await loadDir("");
  } catch (error) {
    ideMessage.set(String(error));
  }
}

export async function startFsEvents() {
  registerAgentEditHook((path) => {
    refreshDirs([path]);
    reloadFromDisk(path);
  });
  if (unlisten) return;
  unlisten = await listen("ide-fs", (event) => {
    const paths = event.payload?.paths ?? [];
    refreshDirs(paths);
    for (const path of paths) reloadFromDisk(path);
  });
}

export async function ensureIndex() {
  if (get(fileIndex).length > 0) return;
  try {
    fileIndex.set(await ideIndex());
  } catch (error) {
    ideMessage.set(String(error));
  }
}
