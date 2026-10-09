const KEY = "stynx.recentWorkspaces";
const MAX = 12;

export function recentWorkspaces() {
  try {
    const raw = localStorage.getItem(KEY);
    const list = raw ? JSON.parse(raw) : [];
    return Array.isArray(list) ? list.filter((path) => path && path !== "/") : [];
  } catch {
    return [];
  }
}

export function rememberWorkspace(path) {
  if (!path || path === "/") return recentWorkspaces();
  const list = [path, ...recentWorkspaces().filter((p) => p !== path)].slice(0, MAX);
  try {
    localStorage.setItem(KEY, JSON.stringify(list));
  } catch {
  }
  return list;
}

export function forgetWorkspace(path) {
  const list = recentWorkspaces().filter((p) => p !== path);
  try {
    localStorage.setItem(KEY, JSON.stringify(list));
  } catch {
  }
  return list;
}
