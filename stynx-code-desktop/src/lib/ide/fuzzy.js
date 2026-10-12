export function fuzzyScore(query, candidate) {
  if (!query) return 1;
  const q = query.toLowerCase();
  const c = candidate.toLowerCase();
  const file = c.slice(c.lastIndexOf("/") + 1);
  if (file.startsWith(q)) return 1000 - file.length;
  if (file.includes(q)) return 800 - file.length;
  let score = 0;
  let last = -1;
  for (const ch of q) {
    const at = c.indexOf(ch, last + 1);
    if (at < 0) return 0;
    score += at === last + 1 ? 6 : 1;
    if (at === 0 || c[at - 1] === "/" || c[at - 1] === "_" || c[at - 1] === "-") score += 4;
    last = at;
  }
  return score + 100 - Math.min(c.length, 100) / 10;
}

export function fuzzyFilter(query, candidates, limit = 50) {
  return candidates
    .map((path) => ({ path, score: fuzzyScore(query.trim(), path) }))
    .filter((entry) => entry.score > 0)
    .sort((a, b) => b.score - a.score || a.path.localeCompare(b.path))
    .slice(0, limit)
    .map((entry) => entry.path);
}
