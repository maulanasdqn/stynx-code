import { get, writable } from "svelte/store";

const MODE_KEY = "stynx.viewMode";

function storedMode() {
  try {
    return localStorage.getItem(MODE_KEY) === "code" ? "code" : "chat";
  } catch {
    return "chat";
  }
}

export const viewMode = writable(storedMode());
viewMode.subscribe((mode) => {
  try {
    localStorage.setItem(MODE_KEY, mode);
  } catch {
    return;
  }
});

export const ideRoot = writable(null);
export const tree = writable(new Map());
export const expanded = writable(new Set([""]));
export const tabs = writable([]);
export const activePath = writable(null);
export const quickOpenVisible = writable(false);
export const fileIndex = writable([]);
export const ideMessage = writable("");
export const namePrompt = writable(null);
export const activeSidePanel = writable("explorer");

const hooks = { agentEdit: null };

export function registerAgentEditHook(fn) {
  hooks.agentEdit = fn;
}

export const pendingOpen = writable(null);

export function openInEditor(path) {
  pendingOpen.set(path);
  viewMode.set("code");
}

export function toggleViewMode() {
  viewMode.update((mode) => (mode === "code" ? "chat" : "code"));
}

export function relativeToRoot(path) {
  const root = get(ideRoot);
  if (!path || !root) return null;
  if (!path.startsWith("/")) return path;
  return path.startsWith(`${root}/`) ? path.slice(root.length + 1) : null;
}

export function notifyAgentEdit(path) {
  const relative = relativeToRoot(path);
  if (relative && hooks.agentEdit) hooks.agentEdit(relative);
}

export const terminalVisible = writable(false);
export const terminals = writable([]);
export const activeTerminal = writable(null);

export const gitState = writable({ isRepo: false, prefix: "", staged: [], unstaged: [], conflicted: [] });
export const gitBusy = writable(false);
export const commitMessage = writable("");

export const diagnostics = writable(new Map());
export const lspStatus = writable(new Map());
export const revealTarget = writable(null);

export const installLogs = writable(new Map());
