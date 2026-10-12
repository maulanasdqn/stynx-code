import { get } from "svelte/store";
import { activePath, activeSidePanel, quickOpenVisible, viewMode } from "./ide-stores.js";
import { closeTab, saveFile } from "./documents.js";
import { toggleTerminalPanel } from "./terminals.js";

function handle(event) {
  if (get(viewMode) !== "code" || !(event.metaKey || event.ctrlKey) || event.altKey) return;
  const key = event.key.toLowerCase();
  if (event.shiftKey) {
    if (key === "f") {
      event.preventDefault();
      activeSidePanel.set("search");
    }
    return;
  }
  if (key === "s") {
    event.preventDefault();
    saveFile();
  } else if (key === "p") {
    event.preventDefault();
    quickOpenVisible.set(true);
  } else if (key === "`") {
    event.preventDefault();
    toggleTerminalPanel();
  } else if (key === "w" && get(activePath)) {
    event.preventDefault();
    closeTab(get(activePath));
  }
}

export function installKeybindings() {
  window.addEventListener("keydown", handle, true);
  return () => window.removeEventListener("keydown", handle, true);
}
