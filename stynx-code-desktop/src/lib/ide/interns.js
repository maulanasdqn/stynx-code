import { writable } from "svelte/store";
import { internConfigs, internDelete, internSave, internStatus } from "./ide-api.js";
import { ideMessage } from "./ide-stores.js";
import { mcpPendingReload } from "./mcp.js";
import { saveInternKey } from "../session-actions.js";
import { confirmDestructive } from "../native.js";

export const PROVIDERS = [
  { id: "deepseek", label: "DeepSeek", keyEnv: "DEEPSEEK_API_KEY", baseUrl: "https://api.deepseek.com/v1" },
  { id: "openrouter", label: "OpenRouter", keyEnv: "OPENROUTER_API_KEY", baseUrl: "https://openrouter.ai/api/v1" },
  { id: "openai", label: "OpenAI", keyEnv: "OPENAI_API_KEY", baseUrl: "https://api.openai.com/v1" },
  { id: "qwen", label: "Qwen", keyEnv: "QWEN_API_KEY", baseUrl: "https://dashscope-intl.aliyuncs.com/compatible-mode/v1" },
  { id: "custom", label: "Custom (OpenAI-compatible)", keyEnv: "", baseUrl: "" },
];

export const internRows = writable([]);
export const internEditor = writable(null);

export async function refreshInterns() {
  try {
    const [status, configs] = await Promise.all([internStatus(), internConfigs()]);
    const configured = new Map(configs.map((entry) => [entry.name, entry]));
    internRows.set(status.map((info) => ({ ...info, config: configured.get(info.name) ?? null })));
    const listed = new Set(status.map((info) => info.name));
    internRows.update((rows) => [
      ...rows,
      ...configs.filter((c) => !listed.has(c.name)).map((config) => ({ name: config.name, provider: config.provider, model: config.model, description: config.description ?? "", keyEnv: config.apiKeyEnv ?? "", available: false, config })),
    ]);
  } catch (error) {
    ideMessage.set(String(error));
  }
}

export async function saveIntern(entry, previous, apiKey) {
  await internSave(entry, previous);
  const keyEnv = entry.apiKeyEnv || PROVIDERS.find((p) => p.id === entry.provider)?.keyEnv;
  if (apiKey?.trim() && keyEnv) await saveInternKey(keyEnv, apiKey);
  mcpPendingReload.set(true);
  ideMessage.set(`Saved intern ${entry.name}`);
  await refreshInterns();
}

export async function setInternKey(row, apiKey) {
  if (!row.keyEnv || !apiKey.trim()) return;
  await saveInternKey(row.keyEnv, apiKey);
  mcpPendingReload.set(true);
  await refreshInterns();
}

export async function deleteIntern(name) {
  const ok = await confirmDestructive(`Remove intern "${name}"?`, "It will be removed from ~/.stynx/settings.json. Its API key stays in keys.json.", "Remove");
  if (!ok) return;
  try {
    await internDelete(name);
    mcpPendingReload.set(true);
    await refreshInterns();
  } catch (error) {
    ideMessage.set(String(error));
  }
}
