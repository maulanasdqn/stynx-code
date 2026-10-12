import { writable } from "svelte/store";
import { ideSearch } from "./ide-api.js";
import { revealTarget } from "./ide-stores.js";
import { openFile } from "./documents.js";

export const searchState = writable({ query: "", caseSensitive: false, regex: false });
export const searchResults = writable({ files: [], truncated: false, error: "", busy: false });

let timer = null;
let ticket = 0;

export function runSearch({ query, caseSensitive, regex }, delay = 250) {
  clearTimeout(timer);
  timer = setTimeout(async () => {
    const mine = ++ticket;
    if (!query) {
      searchResults.set({ files: [], truncated: false, error: "", busy: false });
      return;
    }
    searchResults.update((r) => ({ ...r, busy: true, error: "" }));
    try {
      const result = await ideSearch(query, caseSensitive, regex);
      if (mine === ticket) searchResults.set({ ...result, error: "", busy: false });
    } catch (error) {
      if (mine === ticket) searchResults.set({ files: [], truncated: false, error: String(error), busy: false });
    }
  }, delay);
}

export async function openMatch(path, match) {
  await openFile(path);
  revealTarget.set({ path, line: match.line, column: match.column });
}
