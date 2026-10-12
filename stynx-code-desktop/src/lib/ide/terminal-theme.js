import { opaque, resolveVarHex, luminance } from "./color.js";

export function terminalTheme() {
  const background = resolveVarHex("--text-bg", "#1e1e1e");
  const dark = luminance(background) < 0.5;
  const accent = resolveVarHex("--accent", "#0a84ff");
  const red = resolveVarHex("--red", "#ff453a");
  const orange = resolveVarHex("--orange", "#ff9f0a");
  const green = resolveVarHex("--green", "#32d74b");
  const ansi = dark
    ? { black: "#1e1e1e", yellow: "#ffd60a", cyan: "#64d2ff", white: "#d1d1d6", brightBlack: "#636366", brightWhite: "#ffffff" }
    : { black: "#1d1d1f", yellow: "#a05a00", cyan: "#0071a4", white: "#8e8e93", brightBlack: "#636366", brightWhite: "#1d1d1f" };
  return {
    background,
    foreground: opaque("--label", "#e5e5ea"),
    cursor: accent,
    cursorAccent: background,
    selectionBackground: `${accent}55`,
    red,
    green,
    blue: accent,
    magenta: orange,
    brightRed: red,
    brightGreen: green,
    brightYellow: ansi.yellow,
    brightBlue: accent,
    brightMagenta: orange,
    brightCyan: ansi.cyan,
    ...ansi,
  };
}
