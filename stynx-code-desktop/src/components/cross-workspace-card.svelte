<script>
  import Icon from "./icon.svelte";

  export let tool;
  export let incoming;
</script>

<div class="card glass panel">
  <div class="head">
    <span class="glyph">
      <Icon name={incoming ? "tray.and.arrow.down.fill" : "paperplane.fill"} size={12} />
    </span>
    {#if incoming}
      <span class="incoming">Incoming from another workspace</span>
    {:else}
      <span class="route">
        <span class="chip">this workspace</span>
        <span class="arrow"><Icon name="arrow.right" size={10} weight={2.2} /></span>
        <span class="chip">{tool.title || "workspace"}</span>
      </span>
    {/if}
    <span class="spacer"></span>
    {#if tool.running}
      <span class="status"><span class="spinner"></span>{incoming ? "working…" : "waiting…"}</span>
    {:else if tool.isError}
      <span class="warn"><Icon name="exclamationmark.triangle.fill" size={12} /></span>
    {:else}
      <span class="ok"><Icon name="checkmark.circle.fill" size={12} /></span>
    {/if}
  </div>
  {#if tool.subtitle}
    <div class="task selectable">{tool.subtitle}</div>
  {/if}
  {#if !incoming && !tool.running && tool.detail}
    <div class="reply selectable">↩ {tool.detail.slice(0, 400)}</div>
  {/if}
</div>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    border-radius: 12px;
    border-color: rgba(var(--accent-rgb), 0.3);
  }

  .head {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .glyph {
    width: 24px;
    height: 24px;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    background: rgba(var(--accent-rgb), 0.15);
  }

  .incoming {
    font-size: var(--caption);
    font-weight: 600;
    color: var(--secondary);
  }

  .route {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .chip {
    font-size: var(--caption);
    font-weight: 500;
    padding: 3px 8px;
    border-radius: 999px;
    background: rgba(var(--fg-rgb), 0.06);
  }

  .arrow {
    color: var(--secondary);
  }

  .spacer {
    flex: 1;
  }

  .status {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: var(--caption);
    color: var(--secondary);
  }

  .warn {
    color: var(--orange);
  }

  .ok {
    color: var(--green);
  }

  .task {
    font-size: var(--callout);
  }

  .reply {
    font-size: var(--caption);
    color: var(--secondary);
    white-space: pre-wrap;
  }
</style>
