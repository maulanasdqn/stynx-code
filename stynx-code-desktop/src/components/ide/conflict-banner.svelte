<script>
  import Icon from "../icon.svelte";
  import { resolveConflict } from "../../lib/ide/documents.js";

  export let path;
</script>

<div class="banner">
  <span class="icon"><Icon name="exclamationmark.triangle.fill" size={13} /></span>
  <span class="text">This file changed on disk. Your edits haven't been saved.</span>
  <button class="push" on:click={() => resolveConflict(path, "disk")}>Load from Disk</button>
  <button class="push prominent" on:click={() => resolveConflict(path, "mine")}>Keep Mine</button>
</div>

<style>
  .banner {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    font-size: var(--callout);
    background: color-mix(in srgb, var(--orange) 14%, transparent);
    border-bottom: 1px solid color-mix(in srgb, var(--orange) 35%, transparent);
  }

  .icon {
    display: flex;
    color: var(--orange);
  }

  .text {
    flex: 1;
  }
</style>
