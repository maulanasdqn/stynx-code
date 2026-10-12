export function cssColorToHex(color) {
  const value = color.trim();
  if (/^#(?:[0-9a-f]{3,4}|[0-9a-f]{6}|[0-9a-f]{8})$/i.test(value)) {
    return value.length <= 5 ? `#${[...value.slice(1)].map((c) => c + c).join("")}` : value.slice(0, 7);
  }
  const srgb = value.match(/^color\(srgb\s+([^)]+)\)$/i);
  const rgb = value.match(/^rgba?\(([^)]+)\)$/i);
  const match = srgb ?? rgb;
  if (!match) return null;
  const parts = match[1].split(/[\s,/]+/).filter(Boolean).slice(0, 3);
  if (parts.length < 3) return null;
  const channels = parts.map((part) => {
    const percent = part.endsWith("%");
    const number = Number(percent ? part.slice(0, -1) : part);
    const scale = percent ? 255 / 100 : srgb ? 255 : 1;
    return Math.max(0, Math.min(255, Math.round(number * scale)));
  });
  if (channels.some((n) => !Number.isFinite(n))) return null;
  return `#${channels.map((n) => n.toString(16).padStart(2, "0")).join("")}`;
}

export function resolveVarHex(name, fallback) {
  try {
    const probe = document.createElement("span");
    probe.style.color = `var(${name}, ${fallback})`;
    probe.style.position = "absolute";
    probe.style.visibility = "hidden";
    document.body.appendChild(probe);
    const color = getComputedStyle(probe).color;
    probe.remove();
    return cssColorToHex(color) ?? fallback;
  } catch {
    return fallback;
  }
}

function parse(hex) {
  const s = hex.replace("#", "");
  return [0, 2, 4].map((i) => parseInt(s.slice(i, i + 2), 16));
}

export function mixHex(fg, bg, amount) {
  const a = Math.max(0, Math.min(1, amount));
  const [A, B] = [parse(fg), parse(bg)];
  if (A.some(Number.isNaN) || B.some(Number.isNaN)) return fg;
  return `#${A.map((x, i) => Math.round(x * a + B[i] * (1 - a)).toString(16).padStart(2, "0")).join("")}`;
}

export function luminance(hex) {
  const [r, g, b] = parse(hex).map((c) => {
    const v = c / 255;
    return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

export function opaque(name, fallback) {
  const bg = resolveVarHex("--text-bg", "#1e1e1e");
  const probe = document.createElement("span");
  probe.style.color = `var(${name}, ${fallback})`;
  document.body.appendChild(probe);
  const color = getComputedStyle(probe).color;
  probe.remove();
  const alpha = color.match(/rgba\([^)]*,\s*([\d.]+)\)/);
  const hex = cssColorToHex(color) ?? fallback;
  return alpha ? mixHex(hex, bg, Number(alpha[1])) : hex;
}
