import { nextId } from "./stores.js";
import { recordFileChange } from "./changes.js";
import { titleFor, toolInputField } from "./tool-title.js";

const TOOL_DETAIL_CHARS = 2000;
const HIDDEN_TOOLS = new Set(["ask_user_question"]);

export function feedFromTurns(turns) {
  const items = [];
  for (const turn of turns) {
    if (turn.role !== "tool") {
      items.push({ id: nextId(), role: turn.role, text: turn.text, images: turn.images ?? [] });
      continue;
    }
    if (HIDDEN_TOOLS.has(turn.toolName)) continue;
    items.push({ id: nextId(), role: "tool", tool: toolItem(turn) });
    if (!turn.isError && ["file_write", "file_edit"].includes(turn.toolName)) {
      recordFileChange(turn.toolName, turn.toolInput ?? "");
    }
  }
  return items;
}

function toolItem(turn) {
  const name = turn.toolName;
  const input = turn.toolInput ?? "";
  const output = turn.toolOutput ?? "";
  const tool = {
    toolId: `replay-${nextId()}`,
    name,
    title: titleFor(name, input) ?? name,
    subtitle: null,
    detail: output.slice(0, TOOL_DETAIL_CHARS),
    running: false,
    isError: turn.isError,
    badge: null,
  };
  if (name === "message_workspace") {
    tool.title = toolInputField(input, ["target"]) ?? tool.title;
    tool.subtitle = toolInputField(input, ["task"]);
  }
  if (name === "file_write") {
    tool.badge = "A";
    const lines = output.split(" ").find((word) => /^\d+$/.test(word));
    if (lines) tool.stat = `+${lines}`;
  }
  if (name === "file_edit") tool.badge = "M";
  return tool;
}
