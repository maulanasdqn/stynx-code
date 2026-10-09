<script>
  import Icon from "./icon.svelte";
  import { messageQueue } from "../lib/stores.js";
  import { dequeue } from "../lib/messaging.js";
  import { liquid } from "../lib/motion.js";
</script>

{#if $messageQueue.length > 0}
  <div class="queue glass" transition:liquid={{ y: 16, scale: 0.9 }}>
    <div class="head">
      <span>Queue</span>
      <span class="count">{$messageQueue.length}</span>
    </div>
    <div class="items">
      {#each $messageQueue as queued, index}
        <div class="queued">
          <span class="text">{queued.text || "(image)"}</span>
          <button title="Remove" on:click={() => dequeue(index)}>
            <Icon name="xmark" size={9} weight={2.6} />
          </button>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .queue {
    position: absolute;
    right: 16px;
    bottom: calc(var(--composer-h) + 8px);
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 6px;
    padding: 10px;
    border-radius: 16px;
    transform-origin: bottom right;
  }

  .head {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: var(--caption);
    font-weight: 600;
    color: var(--secondary);
  }

  .count {
    font-weight: 700;
    color: white;
    background: var(--accent);
    border-radius: 999px;
    padding: 1px 5px;
  }

  .items {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 260px;
  }

  .queued {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    border-radius: 10px;
    background: rgba(var(--fg-rgb), 0.06);
  }

  .text {
    flex: 1;
    font-size: var(--caption);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  button {
    color: var(--secondary);
    display: flex;
  }
</style>
