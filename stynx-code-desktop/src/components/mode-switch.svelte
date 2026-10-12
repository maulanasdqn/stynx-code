<script>
  import Icon from "./icon.svelte";
  import { viewMode } from "../lib/ide/ide-stores.js";

  const MODES = [
    { id: "chat", label: "Chat", icon: "bubble.left.and.text.bubble.right" },
    { id: "code", label: "Code", icon: "chevron.left.forwardslash.chevron.right" },
  ];
</script>

<div class="switch glass" role="tablist" title="Switch view (⌘⇧E)">
  <span class="thumb" class:right={$viewMode === "code"}></span>
  {#each MODES as mode}
    <button
      class="segment"
      class:active={$viewMode === mode.id}
      role="tab"
      aria-selected={$viewMode === mode.id}
      on:click={() => ($viewMode = mode.id)}
    >
      <Icon name={mode.icon} size={13} weight={1.8} />
      {mode.label}
    </button>
  {/each}
</div>

<style>
  .switch {
    position: relative;
    display: grid;
    grid-template-columns: 1fr 1fr;
    height: 30px;
    padding: 2px;
    border-radius: 999px;
  }

  .thumb {
    position: absolute;
    top: 2px;
    bottom: 2px;
    left: 2px;
    width: calc(50% - 2px);
    border-radius: 999px;
    background: rgba(var(--accent-rgb), 0.9);
    box-shadow: 0 1px 4px rgba(var(--accent-rgb), 0.4);
    transition: transform 0.5s var(--spring);
  }

  .thumb.right {
    transform: translateX(100%);
  }

  .segment {
    position: relative;
    z-index: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    padding: 0 12px;
    font-size: var(--callout);
    font-weight: 500;
    color: var(--secondary);
  }

  .segment.active {
    color: white;
  }
</style>
