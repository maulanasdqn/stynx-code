import { derived } from "svelte/store";
import { gitState } from "./ide-stores.js";

const PRIORITY = { U: 4, D: 3, M: 2, A: 2, R: 2, "?": 1 };

export const gitBadges = derived(gitState, (state) => {
  const badges = new Map();
  const prefix = state.prefix ? `${state.prefix}/` : "";
  const add = (entry) => {
    if (prefix && !entry.path.startsWith(prefix)) return;
    const path = entry.path.slice(prefix.length);
    const label = entry.status;
    const current = badges.get(path);
    if (!current || (PRIORITY[label] ?? 0) > (PRIORITY[current] ?? 0)) badges.set(path, label);
    const parts = path.split("/");
    for (let i = 1; i < parts.length; i++) {
      const dir = parts.slice(0, i).join("/");
      if (!badges.has(dir)) badges.set(dir, "•");
    }
  };
  for (const entry of [...state.conflicted, ...state.staged, ...state.unstaged]) add(entry);
  return badges;
});
