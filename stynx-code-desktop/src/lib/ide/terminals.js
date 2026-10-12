import { get } from "svelte/store";
import { Channel } from "@tauri-apps/api/core";
import { activeTerminal, ideMessage, terminals, terminalVisible } from "./ide-stores.js";
import { terminalClose, terminalOpen, terminalResize, terminalWrite } from "./ide-api.js";

const sessions = new Map();
let counter = 0;

export async function newTerminal(shell = null, size = { cols: 80, rows: 24 }) {
  const key = ++counter;
  const channel = new Channel();
  const session = { key, id: null, listeners: new Set(), buffer: [], exited: false };
  channel.onmessage = (event) => {
    if (event.type === "output") {
      if (session.listeners.size === 0) session.buffer.push(event.data);
      for (const listener of session.listeners) listener(event.data);
    } else if (event.type === "exit") {
      session.exited = true;
      terminals.update((list) => list.map((t) => (t.key === key ? { ...t, exited: true } : t)));
    }
  };
  try {
    session.id = await terminalOpen(shell, size.cols, size.rows, channel);
  } catch (error) {
    ideMessage.set(String(error));
    return null;
  }
  sessions.set(key, session);
  const label = (shell ?? "").split("/").pop() || "shell";
  terminals.update((list) => [...list, { key, label: `${label} ${list.length + 1}`, exited: false }]);
  activeTerminal.set(key);
  terminalVisible.set(true);
  return key;
}

export function subscribe(key, listener) {
  const session = sessions.get(key);
  if (!session) return () => {};
  for (const chunk of session.buffer.splice(0)) listener(chunk);
  session.listeners.add(listener);
  return () => session.listeners.delete(listener);
}

export function write(key, data) {
  const session = sessions.get(key);
  if (session?.id != null && !session.exited) terminalWrite(session.id, data).catch(() => {});
}

export function resize(key, cols, rows) {
  const session = sessions.get(key);
  if (session?.id != null && !session.exited) terminalResize(session.id, cols, rows).catch(() => {});
}

export async function closeTerminal(key) {
  const session = sessions.get(key);
  sessions.delete(key);
  if (session?.id != null) await terminalClose(session.id).catch(() => {});
  const list = get(terminals).filter((t) => t.key !== key);
  terminals.set(list);
  if (get(activeTerminal) === key) activeTerminal.set(list.at(-1)?.key ?? null);
  if (list.length === 0) terminalVisible.set(false);
}

export async function toggleTerminalPanel() {
  if (get(terminalVisible)) {
    terminalVisible.set(false);
  } else if (get(terminals).length === 0) {
    await newTerminal();
  } else {
    terminalVisible.set(true);
  }
}

export async function closeAllTerminals() {
  for (const { key } of get(terminals)) await closeTerminal(key);
}
