<script>
  import { ICONS } from "../lib/icons.js";

  export let name;
  export let size = 13;
  export let weight = 1.8;

  $: raw = ICONS[name] ?? ICONS.doc;
  $: filled = raw.startsWith("fill:");
  $: markup = filled ? raw.slice(5) : raw;
</script>

<svg
  class="icon"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill={filled ? "currentColor" : "none"}
  stroke={filled ? "none" : "currentColor"}
  stroke-width={weight}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {@html markup}
</svg>

<style>
  .icon {
    flex-shrink: 0;
    display: block;
  }
</style>
