import { writable } from "svelte/store";
import { lspInstall, lspServers, lspUninstall } from "./ide-api.js";
import { ideMessage, installLogs } from "./ide-stores.js";
import { reattachLanguageServers } from "./documents.js";

export const servers = writable([]);
export const serverBusy = writable(new Set());

export async function refreshServers() {
  try {
    servers.set(await lspServers());
  } catch (error) {
    ideMessage.set(String(error));
  }
}

async function run(id, action, verb) {
  serverBusy.update((set) => new Set(set).add(id));
  installLogs.update((map) => new Map(map).set(id, []));
  try {
    await action(id);
    ideMessage.set(`${verb} ${id} language server`);
    reattachLanguageServers();
  } catch (error) {
    ideMessage.set(String(error));
  } finally {
    serverBusy.update((set) => {
      const next = new Set(set);
      next.delete(id);
      return next;
    });
    refreshServers();
  }
}

export const installServer = (id) => run(id, lspInstall, "Installed");
export const uninstallServer = (id) => run(id, lspUninstall, "Removed");
