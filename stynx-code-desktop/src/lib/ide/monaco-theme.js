import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { luminance, mixHex, opaque, resolveVarHex } from "./color.js";

export const THEME = "stynx";

function palette() {
  const bg = resolveVarHex("--text-bg", "#1e1e1e");
  return {
    bg,
    fg: opaque("--label", "#e5e5ea"),
    dim: opaque("--secondary", "#98989d"),
    faint: opaque("--tertiary", "#5a5a5e"),
    border: opaque("--separator", "#3a3a3c"),
    accent: resolveVarHex("--accent", "#0a84ff"),
    green: resolveVarHex("--green", "#32d74b"),
    red: resolveVarHex("--red", "#ff453a"),
    orange: resolveVarHex("--orange", "#ff9f0a"),
    blue: resolveVarHex("--blue", "#0a84ff"),
  };
}

function tokenRules(p, dark) {
  const strip = (hex) => hex.replace("#", "");
  const teal = dark ? "64d2ff" : "0071a4";
  return [
    { token: "keyword", foreground: strip(p.accent) },
    { token: "keyword.flow", foreground: strip(p.accent) },
    { token: "keyword.control", foreground: strip(p.accent) },
    { token: "type", foreground: teal },
    { token: "type.identifier", foreground: teal },
    { token: "string", foreground: strip(p.orange) },
    { token: "number", foreground: dark ? "a5d6ff" : "1c00cf" },
    { token: "regexp", foreground: strip(p.red) },
    { token: "comment", foreground: strip(p.dim), fontStyle: "italic" },
    { token: "annotation", foreground: teal },
    { token: "attribute.name", foreground: teal },
    { token: "tag", foreground: strip(p.accent) },
    { token: "metatag", foreground: strip(p.accent) },
  ];
}

const SYMBOL_ICONS = [
  "method", "function", "constructor", "class", "struct", "interface", "event",
  "operator", "typeParameter", "module", "namespace", "enum", "enumMember",
];

function colors(p) {
  const { bg, fg, dim, faint, border, accent, green, red, orange, blue } = p;
  const panel = mixHex(fg, bg, 0.06);
  const hover = mixHex(fg, bg, 0.1);
  const symbols = Object.fromEntries(SYMBOL_ICONS.map((name) => [`symbolIcon.${name}Foreground`, accent]));
  return {
    ...symbols,
    "editor.background": bg,
    "editor.foreground": fg,
    "editorLineNumber.foreground": faint,
    "editorLineNumber.activeForeground": dim,
    "editorGutter.background": bg,
    "editorCursor.foreground": accent,
    "editor.selectionBackground": mixHex(accent, bg, 0.3),
    "editor.inactiveSelectionBackground": mixHex(accent, bg, 0.16),
    "editor.lineHighlightBackground": mixHex(fg, bg, 0.04),
    "editor.lineHighlightBorder": bg,
    "editor.wordHighlightBackground": mixHex(accent, bg, 0.14),
    "editor.findMatchBackground": mixHex(orange, bg, 0.35),
    "editor.findMatchHighlightBackground": mixHex(orange, bg, 0.18),
    "editorIndentGuide.background1": mixHex(fg, bg, 0.07),
    "editorIndentGuide.activeBackground1": mixHex(fg, bg, 0.18),
    "editorBracketMatch.background": mixHex(accent, bg, 0.12),
    "editorBracketMatch.border": mixHex(accent, bg, 0.45),
    "editorBracketHighlight.foreground1": orange,
    "editorBracketHighlight.foreground2": accent,
    "editorBracketHighlight.foreground3": green,
    "editorWidget.background": panel,
    "editorWidget.border": border,
    "editorHoverWidget.background": panel,
    "editorHoverWidget.border": border,
    "editorSuggestWidget.background": panel,
    "editorSuggestWidget.border": border,
    "editorSuggestWidget.selectedBackground": hover,
    "editorSuggestWidget.highlightForeground": accent,
    "editorSuggestWidget.focusHighlightForeground": accent,
    "editorError.foreground": red,
    "editorWarning.foreground": orange,
    "editorInfo.foreground": blue,
    "editorOverviewRuler.border": bg,
    "editorOverviewRuler.errorForeground": red,
    "editorOverviewRuler.warningForeground": orange,
    "diffEditor.insertedTextBackground": mixHex(green, bg, 0.2),
    "diffEditor.removedTextBackground": mixHex(red, bg, 0.2),
    "diffEditor.insertedLineBackground": mixHex(green, bg, 0.1),
    "diffEditor.removedLineBackground": mixHex(red, bg, 0.1),
    "diffEditorGutter.insertedLineBackground": mixHex(green, bg, 0.22),
    "diffEditorGutter.removedLineBackground": mixHex(red, bg, 0.22),
    "diffEditor.diagonalFill": border,
    "input.background": mixHex(fg, bg, 0.05),
    "input.border": border,
    "inputOption.activeBorder": accent,
    "list.activeSelectionBackground": hover,
    "list.hoverBackground": mixHex(fg, bg, 0.06),
    "list.highlightForeground": accent,
    "scrollbarSlider.background": mixHex(fg, bg, 0.14),
    "scrollbarSlider.hoverBackground": mixHex(fg, bg, 0.24),
    "scrollbarSlider.activeBackground": mixHex(fg, bg, 0.32),
    "scrollbar.shadow": bg,
    "widget.shadow": mixHex("#000000", bg, 0.4),
    "focusBorder": accent,
    "textLink.foreground": accent,
    "menu.background": panel,
    "menu.selectionBackground": hover,
    "progressBar.background": accent,
  };
}

export function applyTheme() {
  const p = palette();
  const dark = luminance(p.bg) < 0.5;
  monaco.editor.defineTheme(THEME, {
    base: dark ? "vs-dark" : "vs",
    inherit: true,
    rules: tokenRules(p, dark),
    colors: colors(p),
  });
  monaco.editor.setTheme(THEME);
}

export function watchColorScheme() {
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    requestAnimationFrame(applyTheme);
  });
}
