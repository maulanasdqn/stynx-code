import { get } from "svelte/store";
import { activePath, quickOpenVisible, viewMode } from "./ide-stores.js";
import { closeTab, saveFile } from "./documents.js";

function handle(event) {
  if (get(viewMode) !== "code" || !(event.metaKey || event.ctrlKey) || event.shiftKey || event.altKey) return;
  const key = event.key.toLowerCase();
  if (key === "s") {
    event.preventDefault();
    saveFile();
  } else if (key === "p") {
    event.preventDefault();
    quickOpenVisible.set(true);
  } else if (key === "w" && get(activePath)) {
    event.preventDefault();
    closeTab(get(activePath));
  }
}

export function installKeybindings() {
  window.addEventListener("keydown", handle, true);
  return () => window.removeEventListener("keydown", handle, true);
}
