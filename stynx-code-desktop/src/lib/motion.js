export const spring = (t) => (t >= 1 ? 1 : 1 - Math.exp(-6.5 * t) * Math.cos(9 * t));

export const smooth = (t) => (t >= 1 ? 1 : 1 - Math.exp(-7 * t) * (1 + 7 * t));

const reduced = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

export function installEasings() {
  const sample = (fn) =>
    `linear(${Array.from({ length: 41 }, (_, i) => fn(i / 40).toFixed(4)).join(", ")})`;
  const root = document.documentElement.style;
  root.setProperty("--spring", sample(spring));
  root.setProperty("--smooth", sample(smooth));
}

export function liquid(node, { y = 10, scale = 0.97, duration = 520, delay = 0 } = {}) {
  if (reduced()) return { duration: 0 };
  return {
    delay,
    duration,
    css: (t) => {
      const e = spring(t);
      const s = smooth(t);
      return `opacity:${Math.min(1, s * 1.4)};transform:translateY(${(1 - e) * y}px) scale(${scale + (1 - scale) * e});filter:blur(${(1 - s) * 6}px)`;
    },
  };
}

export function unfold(node, { duration = 480, axis = "width" } = {}) {
  if (reduced()) return { duration: 0 };
  const size = axis === "width" ? node.offsetWidth : node.offsetHeight;
  return {
    duration,
    css: (t) => {
      const s = smooth(t);
      return `${axis}:${s * size}px;opacity:${s};overflow:hidden;filter:blur(${(1 - s) * 8}px)`;
    },
  };
}

export function trackSpecular(root) {
  const move = (event) => {
    const glass = event.target.closest?.(".glass");
    if (!glass) return;
    const rect = glass.getBoundingClientRect();
    glass.style.setProperty("--mx", `${((event.clientX - rect.left) / rect.width) * 100}%`);
    glass.style.setProperty("--my", `${((event.clientY - rect.top) / rect.height) * 100}%`);
  };
  root.addEventListener("pointermove", move, { passive: true });
  return { destroy: () => root.removeEventListener("pointermove", move) };
}
