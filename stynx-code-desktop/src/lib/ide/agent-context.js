import { derived, get } from "svelte/store";
import { composerDraft, composerFocus } from "../stores.js";
import { agentVisible, editorContext, includeContext } from "./ide-stores.js";
import { languageIdFor } from "./languages.js";

const MAX_SELECTION = 12000;

export function buildContext(ctx) {
  if (!ctx?.path) return null;
  const name = ctx.path.split("/").pop();
  const hasSelection = ctx.text && ctx.text.trim().length > 0;
  const lines = hasSelection ? (ctx.startLine === ctx.endLine ? `L${ctx.startLine}` : `L${ctx.startLine}–${ctx.endLine}`) : null;
  const label = lines ? `${name} · ${lines}` : name;
  let prompt = `[Context: the user is working in \`${ctx.path}\``;
  if (hasSelection) {
    const fence = languageIdFor(ctx.path) ?? "";
    const code = ctx.text.length > MAX_SELECTION ? `${ctx.text.slice(0, MAX_SELECTION)}\n…(truncated)` : ctx.text;
    prompt += ` and selected lines ${ctx.startLine}–${ctx.endLine}:]\n\`\`\`${fence}\n${code}\n\`\`\``;
  } else {
    prompt += ctx.line ? `, cursor at line ${ctx.line}.]` : ".]";
  }
  return { label, prompt, hasSelection, name };
}

export const agentContext = derived([editorContext, includeContext], ([$ctx, $include]) => ($include ? buildContext($ctx) : null));

export function askAgent(prompt = null) {
  agentVisible.set(true);
  includeContext.set(true);
  if (prompt !== null) composerDraft.set(prompt);
  composerFocus.update((n) => n + 1);
}

export function suggestionsFor(ctx) {
  const name = ctx?.path?.split("/").pop();
  if (!name) {
    return ["Give me a tour of this project", "Where should I start reading?", "Find likely bugs", "What changed recently?"];
  }
  if (ctx.text?.trim()) {
    return ["Explain this code", "Find bugs in this selection", "Refactor this to be clearer", "Write tests for this"];
  }
  return [`Explain ${name}`, `Find bugs in ${name}`, `Write tests for ${name}`, `How is ${name} used?`];
}

export const hasEditorContext = () => Boolean(get(editorContext)?.path);
