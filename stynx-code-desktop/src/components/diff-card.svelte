<script>
  import Icon from "./icon.svelte";
  import DiffLines from "./diff-lines.svelte";
  import { composerDraft } from "../lib/stores.js";
  import { diffQuote } from "../lib/diff.js";
  import { slide } from "svelte/transition";
  import { smooth, liquid } from "../lib/motion.js";

  export let change;
  export let expandedByDefault;

  let expanded = null;
  let selected = new Set();

  $: isExpanded = expanded ?? expandedByDefault;
  $: badge = change.kind === "file_write" ? "A" : "M";
  $: selectedLines = (change.diff ?? []).filter((line) => selected.has(line.id));

  function toggleExpanded() {
    expanded = !isExpanded;
    if (!expanded) selected = new Set();
  }

  function toggleLine(id) {
    const next = new Set(selected);
    next.has(id) ? next.delete(id) : next.add(id);
    selected = next;
  }

  function quote(event) {
    event.stopPropagation();
    const block = diffQuote(change.path, selectedLines);
    $composerDraft = $composerDraft ? `${$composerDraft}\n${block}` : block;
    selected = new Set();
  }
</script>

<div class="card glass panel">
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="head" on:click={toggleExpanded}>
    <span class="dim chevron" class:open={isExpanded}><Icon name="chevron.right" size={9} weight={2.4} /></span>
    <span class="dim"><Icon name="doc.text" size={12} /></span>
    <span class="name mono ellipsis">{change.name}</span>
    <span class="spacer"></span>
    {#if selectedLines.length > 0}
      <button class="quote" on:click={quote} in:liquid={{ y: 0, scale: 0.7, duration: 460 }}>
        <Icon name="quote.bubble" size={11} />
        Quote {selectedLines.length} line{selectedLines.length === 1 ? "" : "s"}
      </button>
    {/if}
    {#if change.adds > 0}<span class="adds">+{change.adds}</span>{/if}
    {#if change.removes > 0}<span class="removes">-{change.removes}</span>{/if}
    <span class="badge" class:added={badge === "A"}>{badge}</span>
  </div>
  {#if isExpanded}
    <div transition:slide={{ duration: 420, easing: smooth }}>
      <div class="divider"></div>
      <DiffLines {change} {selected} onToggle={toggleLine} />
    </div>
  {/if}
</div>

<style>
  .card {
    border-radius: 14px;
    overflow: hidden;
    flex-shrink: 0;
    animation: surface 0.6s var(--spring) backwards;
  }

  .head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    font-size: var(--caption);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .dim {
    color: var(--secondary);
  }

  .chevron {
    transition: transform 0.5s var(--spring);
  }

  .chevron.open {
    transform: rotate(90deg);
  }

  .name {
    font-size: var(--callout);
    font-weight: 400;
    min-width: 0;
  }

  .spacer {
    flex: 1;
    min-width: 6px;
  }

  .quote {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border-radius: 999px;
    font-weight: 600;
    white-space: nowrap;
    color: var(--accent);
    background: rgba(var(--accent-rgb), 0.2);
  }

  .adds {
    color: var(--green);
  }

  .removes {
    color: var(--red);
  }

  .badge {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    color: white;
    background: var(--blue);
    flex-shrink: 0;
  }

  .badge.added {
    background: var(--green);
  }

  .divider {
    height: 1px;
    background: var(--separator);
  }
</style>
