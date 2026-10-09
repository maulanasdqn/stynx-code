<script>
  import Icon from "./icon.svelte";

  export let tool;

  function toolBadge(name) {
    switch (name) {
      case "file_write":
        return { symbol: "doc.badge.plus", accent: true };
      case "file_edit":
        return { symbol: "pencil", accent: true };
      case "read":
        return { symbol: "doc.text", accent: false };
      case "bash":
        return { symbol: "terminal", accent: false };
      case "glob":
        return { symbol: "magnifyingglass", accent: false };
      case "grep":
        return { symbol: "text.magnifyingglass", accent: false };
      case "web_fetch":
      case "web_search":
        return { symbol: "globe", accent: false };
      case "todo_write":
      case "todo_read":
        return { symbol: "checklist", accent: false };
      default:
        return name.startsWith("delegate_to_")
          ? { symbol: "person.2", accent: true }
          : { symbol: "gearshape", accent: false };
    }
  }

  $: glyph = toolBadge(tool.name);
</script>

<div class="card" class:accent={glyph.accent}>
  <span class="glyph"><Icon name={glyph.symbol} size={13} weight={2.2} /></span>
  <span class="text">
    <span class="title mono ellipsis">{tool.title || tool.name}</span>
    <span class="name">{tool.name}</span>
  </span>
  <span class="spacer"></span>
  {#if tool.stat}<span class="stat">{tool.stat}</span>{/if}
  {#if tool.running}
    <span class="spinner"></span>
  {:else if tool.isError}
    <span class="warn"><Icon name="exclamationmark.triangle.fill" size={12} /></span>
  {:else if tool.badge}
    <span class="badge" class:added={tool.badge === "A"}>{tool.badge}</span>
  {:else}
    <span class="done"><Icon name="checkmark" size={10} weight={3} /></span>
  {/if}
</div>

<style>
  .card {
    --tint: var(--secondary);
    --bar: rgba(var(--fg-rgb), 0.25);
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 12px 9px 15px;
    border-radius: 10px;
    background: rgba(var(--fg-rgb), 0.05);
    overflow: hidden;
    transition:
      background-color 0.3s var(--smooth),
      transform 0.5s var(--spring);
  }

  .card:hover {
    background: rgba(var(--fg-rgb), 0.075);
    transform: translateX(2px);
  }

  .card.accent {
    --tint: var(--accent);
    --bar: var(--accent);
  }

  .card::before {
    content: "";
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--bar);
  }

  .glyph {
    width: 22px;
    height: 22px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: var(--tint);
    background: color-mix(in srgb, var(--tint) 14%, transparent);
  }

  .text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .title {
    font-size: var(--callout);
    color: var(--label);
  }

  .name {
    font-size: var(--caption);
    color: var(--tertiary);
  }

  .spacer {
    flex: 1;
    min-width: 6px;
  }

  .stat {
    font-size: var(--caption);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    color: var(--green);
  }

  .warn {
    color: var(--orange);
  }

  .done {
    color: var(--green);
  }

  .badge {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--caption);
    font-weight: 700;
    color: white;
    background: var(--blue);
    flex-shrink: 0;
  }

  .badge.added {
    background: var(--green);
  }
</style>
