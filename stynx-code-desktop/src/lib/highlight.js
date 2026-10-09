// Port of CodeViewer.swift's `highlight(_:ext:)`: a tiny regex colourer.
// Passes run in the same order as Swift so later passes win (strings > keywords > numbers).

const KEYWORDS = [
  "fn", "let", "mut", "pub", "struct", "enum", "impl", "trait", "use", "mod", "match",
  "if", "else", "for", "while", "loop", "return", "self", "async", "await", "move",
  "func", "var", "class", "extension", "guard", "import", "static", "private", "public",
  "const", "function", "def", "yield", "true", "false", "nil", "null", "None",
  "where", "in", "as", "is", "try", "throw", "throws", "do", "case", "switch", "default",
];

const PASSES = [
  { regex: /\b[0-9]+(\.[0-9]+)?\b/g, cls: "hl-num" },
  { regex: new RegExp(`\\b(${KEYWORDS.join("|")})\\b`, "g"), cls: "hl-kw" },
  { regex: /"[^"]*"/g, cls: "hl-str" },
];

const escape = (text) =>
  text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");

export function highlight(line) {
  const trimmed = line.trimStart();
  if (trimmed.startsWith("//") || trimmed.startsWith("#") || trimmed.startsWith("*")) {
    return `<span class="hl-comment">${escape(line)}</span>`;
  }

  const classes = new Array(line.length).fill(null);
  for (const { regex, cls } of PASSES) {
    for (const match of line.matchAll(regex)) {
      classes.fill(cls, match.index, match.index + match[0].length);
    }
  }

  let html = "";
  let start = 0;
  for (let i = 1; i <= line.length; i++) {
    if (i === line.length || classes[i] !== classes[start]) {
      const chunk = escape(line.slice(start, i));
      html += classes[start] ? `<span class="${classes[start]}">${chunk}</span>` : chunk;
      start = i;
    }
  }
  return html;
}
