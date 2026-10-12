import { writable, get } from "svelte/store";
import { createHighlighterCore } from "shiki/core";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";
import { monaco } from "./monaco-setup.js";
import { languageIdFor, loaderFor } from "./languages.js";
import { ideMessage } from "./ide-stores.js";

const failed = (id) => (error) => {
  ideMessage.set(`Syntax for ${id} unavailable: ${error?.message ?? error}`);
  return false;
};

const KEY = "stynx.syntax.installed";
const AUTO_KEY = "stynx.syntax.auto";
const MAX_LINE = 20000;

function readSet() {
  try {
    return new Set(JSON.parse(localStorage.getItem(KEY) ?? "[]"));
  } catch {
    return new Set();
  }
}

export const installedLanguages = writable(readSet());
export const autoInstall = writable(localStorage.getItem(AUTO_KEY) !== "off");
installedLanguages.subscribe((set) => localStorage.setItem(KEY, JSON.stringify([...set])));
autoInstall.subscribe((on) => localStorage.setItem(AUTO_KEY, on ? "on" : "off"));

let highlighterPromise = null;
const ready = new Set();

const highlighter = () =>
  (highlighterPromise ??= createHighlighterCore({ themes: [], langs: [], engine: createOnigurumaEngine(import("shiki/wasm")) }));

class State {
  constructor(stack) {
    this.stack = stack;
  }
  clone() {
    return new State(this.stack);
  }
  equals(other) {
    return other instanceof State && (other.stack === this.stack || Boolean(this.stack?.equals?.(other.stack)));
  }
}

function provider(grammar) {
  return {
    getInitialState: () => new State(null),
    tokenize(line, state) {
      if (line.length > MAX_LINE) return { tokens: [{ startIndex: 0, scopes: "" }], endState: state };
      const result = grammar.tokenizeLine(line, state.stack, 500);
      const tokens = result.tokens.map((token) => ({
        startIndex: token.startIndex,
        scopes: token.scopes[token.scopes.length - 1] ?? "",
      }));
      return { tokens, endState: new State(result.ruleStack) };
    },
  };
}

async function activate(id) {
  if (ready.has(id)) return true;
  const loader = loaderFor(id);
  if (!loader) return false;
  const shiki = await highlighter();
  await shiki.loadLanguage(loader);
  if (!monaco.languages.getLanguages().some((lang) => lang.id === id)) monaco.languages.register({ id });
  monaco.languages.setTokensProvider(id, provider(shiki.getLanguage(id)));
  ready.add(id);
  return true;
}

export async function installLanguage(id) {
  const ok = await activate(id).catch(failed(id));
  if (ok) installedLanguages.update((set) => new Set(set).add(id));
  return ok;
}

export function uninstallLanguage(id) {
  installedLanguages.update((set) => {
    const next = new Set(set);
    next.delete(id);
    return next;
  });
  for (const model of monaco.editor.getModels()) {
    if (model.getLanguageId() === id) monaco.editor.setModelLanguage(model, "plaintext");
  }
}

export async function languageFor(path) {
  const id = languageIdFor(path);
  if (!id || id === "text") return "plaintext";
  if (get(installedLanguages).has(id)) return (await activate(id).catch(failed(id))) ? id : "plaintext";
  if (get(autoInstall)) return (await installLanguage(id)) ? id : "plaintext";
  return "plaintext";
}

export async function restoreInstalled() {
  for (const id of get(installedLanguages)) await activate(id).catch(failed(id));
}
