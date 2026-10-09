<script>
  import Icon from "../icon.svelte";
  import { sessions, isStreaming } from "../../lib/stores.js";
  import { openSession, startNewSession, removeSession } from "../../lib/session-actions.js";
  import { popupMenu } from "../../lib/native.js";

  function contextMenu(event, summary) {
    event.preventDefault();
    popupMenu(
      [
        { text: "Open", action: () => openSession(summary.id) },
        { text: "Delete", action: () => removeSession(summary.id) },
      ],
      event,
    );
  }
</script>

<section>
  <div class="sb-header">Sessions</div>
  <button class="sb-row" on:click={startNewSession} disabled={$isStreaming}>
    <span class="sb-label-icon"><Icon name="plus.bubble" size={15} /></span>
    New conversation
  </button>
  {#each $sessions as summary (summary.id)}
    <button
      class="sb-row"
      on:click={() => openSession(summary.id)}
      on:contextmenu={(event) => contextMenu(event, summary)}
    >
      <span class="glyph"><Icon name="bubble.left.and.text.bubble.right" size={13} /></span>
      <span class="sb-meta">
        <span class="sb-title ellipsis">{summary.title || "Untitled"}</span>
        <span class="sb-sub">{summary.messageCount} messages</span>
      </span>
    </button>
  {:else}
    <div class="sb-empty">No saved sessions</div>
  {/each}
</section>

<style>
  .glyph {
    color: var(--secondary);
    width: 16px;
    display: flex;
    justify-content: center;
  }
</style>
