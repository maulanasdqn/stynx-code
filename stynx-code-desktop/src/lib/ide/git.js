import { get } from "svelte/store";
import { gitCommit, gitDiff, gitDiscard, gitStage, gitStatus, gitUnstage } from "./ide-api.js";
import { activePath, commitMessage, gitBusy, gitState, ideMessage, tabs } from "./ide-stores.js";
import { confirmDestructive } from "../native.js";

let timer = null;

export function refreshGit(delay = 0) {
  clearTimeout(timer);
  timer = setTimeout(async () => {
    try {
      gitState.set(await gitStatus());
    } catch (error) {
      ideMessage.set(String(error));
    }
  }, delay);
}

export function toWorkspacePath(repoPath) {
  const { prefix } = get(gitState);
  if (!prefix) return repoPath;
  return repoPath.startsWith(`${prefix}/`) ? repoPath.slice(prefix.length + 1) : null;
}

async function mutate(action) {
  gitBusy.set(true);
  try {
    await action();
  } catch (error) {
    ideMessage.set(String(error));
  } finally {
    gitBusy.set(false);
    refreshGit();
  }
}

export const stage = (paths) => mutate(() => gitStage(paths));
export const unstage = (paths) => mutate(() => gitUnstage(paths));

export async function discard(paths) {
  const label = paths.length === 1 ? `"${paths[0].split("/").pop()}"` : `${paths.length} files`;
  const ok = await confirmDestructive(`Discard changes in ${label}?`, "Your uncommitted changes will be lost. Untracked files are deleted.", "Discard");
  if (ok) await mutate(() => gitDiscard(paths));
}

export async function commit() {
  const message = get(commitMessage).trim();
  if (!message || get(gitState).staged.length === 0) return;
  await mutate(async () => {
    await gitCommit(message);
    commitMessage.set("");
    ideMessage.set("Committed");
  });
}

export async function openDiff(path, staged) {
  const key = `diff:${staged ? "staged" : "working"}:${path}`;
  let diff;
  try {
    diff = await gitDiff(path, staged);
  } catch (error) {
    ideMessage.set(String(error));
    return;
  }
  const name = `${path.split("/").pop()} ${staged ? "(Staged)" : "(Working Tree)"}`;
  const tab = { path: key, name, kind: "diff", dirty: false, conflict: false, diff };
  tabs.update((list) => (list.some((t) => t.path === key) ? list.map((t) => (t.path === key ? tab : t)) : [...list, tab]));
  activePath.set(key);
}
