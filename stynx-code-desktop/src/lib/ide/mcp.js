import { get, writable } from "svelte/store";
import { mcpInstall, mcpInstalled, mcpSearch, mcpUninstall } from "./ide-api.js";
import { ideMessage } from "./ide-stores.js";
import { info } from "../stores.js";
import { openWorkspace, startNewSession } from "../session-actions.js";

export const mcpResults = writable([]);
export const mcpCursor = writable(null);
export const mcpLoading = writable(false);
export const mcpServersInstalled = writable([]);
export const mcpPendingReload = writable(false);
export const mcpInstallTarget = writable(null);

let timer = null;
let latest = 0;

export async function refreshInstalled() {
  try {
    mcpServersInstalled.set(await mcpInstalled());
  } catch (error) {
    ideMessage.set(String(error));
  }
}

export function searchRegistry(query, delay = 300) {
  clearTimeout(timer);
  timer = setTimeout(async () => {
    const ticket = ++latest;
    mcpLoading.set(true);
    try {
      const page = await mcpSearch(query, null);
      if (ticket !== latest) return;
      mcpResults.set(page.servers);
      mcpCursor.set(page.nextCursor);
    } catch (error) {
      ideMessage.set(`MCP registry: ${error}`);
    } finally {
      if (ticket === latest) mcpLoading.set(false);
    }
  }, delay);
}

export async function loadMore(query) {
  const cursor = get(mcpCursor);
  if (!cursor || get(mcpLoading)) return;
  mcpLoading.set(true);
  try {
    const page = await mcpSearch(query, cursor);
    mcpResults.update((list) => [...list, ...page.servers]);
    mcpCursor.set(page.nextCursor);
  } finally {
    mcpLoading.set(false);
  }
}

export function buildEntry(plan, values) {
  const fill = (text) => text.replace(/\{([^}]+)\}/g, (_, key) => values[key] ?? "");
  const env = {};
  for (const input of plan.inputs) {
    const value = (values[input.key] ?? "").trim();
    if (input.target === "env" && value) env[input.key] = value;
  }
  return { command: plan.command, args: plan.args.map(fill), env };
}

export async function install(listing, scope, name, values) {
  await mcpInstall(scope, name, buildEntry(listing.plan, values));
  ideMessage.set(`Installed MCP server ${name}`);
  mcpPendingReload.set(true);
  await refreshInstalled();
}

export async function uninstall(item) {
  try {
    await mcpUninstall(item.scope, item.name);
    mcpPendingReload.set(true);
    await refreshInstalled();
  } catch (error) {
    ideMessage.set(String(error));
  }
}

export async function reloadAgent() {
  const current = get(info);
  if (!current) return;
  await openWorkspace(current.workspacePath, current.currentProvider);
  await startNewSession();
  mcpPendingReload.set(false);
  ideMessage.set("Agent reloaded with MCP servers");
}
