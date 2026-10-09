<script>
  import { tick } from "svelte";
  import Icon from "./icon.svelte";
  import DiffCard from "./diff-card.svelte";
  import { changes } from "../lib/stores.js";

  let scroller;

  $: lastId = $changes[$changes.length - 1]?.id;
  $: $changes.length, scrollToBottom();

  async function scrollToBottom() {
    await tick();
    scroller?.scrollTo({ top: scroller.scrollHeight, behavior: "smooth" });
  }
</script>

<div class="panel">
  <div class="header">
    <Icon name="plus.forwardslash.minus" size={14} />
    <span class="title">Diff</span>
    <span class="spacer"></span>
    {#if $changes.length > 0}
      <span class="count">{$changes.length}</span>
    {/if}
  </div>
  {#if $changes.length === 0}
    <div class="empty">
      <span class="empty-icon"><Icon name="doc.text.magnifyingglass" size={30} weight={1.4} /></span>
      <span class="empty-text">No changes yet</span>
    </div>
  {:else}
    <div class="list" bind:this={scroller}>
      {#each $changes as change (change.id)}
        <DiffCard {change} expandedByDefault={change.id === lastId} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .panel {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 14px;
    border-bottom: 1px solid var(--separator);
  }

  .title {
    font-size: var(--headline);
    font-weight: 700;
  }

  .spacer {
    flex: 1;
  }

  .count {
    font-size: var(--caption);
    font-variant-numeric: tabular-nums;
    color: var(--secondary);
    padding: 2px 7px;
    border-radius: 999px;
    background: var(--quaternary);
  }

  .empty {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .empty-icon {
    color: var(--tertiary);
  }

  .empty-text {
    font-size: var(--callout);
    color: var(--secondary);
  }

  .list {
    flex: 1;
    overflow-y: auto;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
</style>
