<script>
  import Icon from "../icon.svelte";
  import { discard, openDiff, stage, toWorkspacePath, unstage } from "../../lib/ide/git.js";
  import { openFile } from "../../lib/ide/documents.js";

  export let entry;
  export let staged = false;
  export let conflicted = false;

  const COLORS = { M: "var(--orange)", A: "var(--green)", "?": "var(--green)", D: "var(--red)", R: "var(--blue)", U: "var(--red)" };

  $: name = entry.path.split("/").pop();
  $: dir = entry.path.includes("/") ? entry.path.slice(0, entry.path.lastIndexOf("/")) : "";
  $: workspacePath = toWorkspacePath(entry.path);
  $: label = entry.status === "?" ? "U" : entry.status;

  function show() {
    if (conflicted || entry.status === "D") {
      if (workspacePath && entry.status !== "D") openFile(workspacePath);
      return;
    }
    openDiff(entry.path, staged);
  }
</script>

<div class="row" role="button" tabindex="0" title={entry.origPath ? `${entry.origPath} → ${entry.path}` : entry.path} on:click={show} on:keydown={(e) => e.key === "Enter" && show()}>
  <span class="name ellipsis" class:deleted={entry.status === "D"}>{name}</span>
  <span class="dir ellipsis">{dir}</span>
  <span class="actions">
    {#if workspacePath && entry.status !== "D"}
      <button title="Open File" on:click|stopPropagation={() => openFile(workspacePath)}><Icon name="doc.text" size={12} /></button>
    {/if}
    {#if !staged && !conflicted}
      <button title="Discard Changes" on:click|stopPropagation={() => discard([entry.path])}><Icon name="arrow.counterclockwise" size={12} /></button>
    {/if}
    {#if staged}
      <button title="Unstage" on:click|stopPropagation={() => unstage([entry.path])}><Icon name="xmark" size={10} weight={2.4} /></button>
    {:else}
      <button title="Stage" on:click|stopPropagation={() => stage([entry.path])}><Icon name="plus" size={12} weight={2.2} /></button>
    {/if}
  </span>
  <span class="status" style="color: {COLORS[entry.status] ?? 'var(--secondary)'}">{label}</span>
</div>

<style>
  .row {
    height: 24px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 8px 0 14px;
    border-radius: 6px;
    font-size: var(--body);
    cursor: default;
  }

  .row:hover {
    background: var(--hover);
  }

  .name {
    flex-shrink: 1;
    min-width: 0;
    max-width: 60%;
    color: var(--label);
  }

  .name.deleted {
    text-decoration: line-through;
    color: var(--secondary);
  }

  .dir {
    flex: 1;
    min-width: 0;
    font-size: var(--caption);
    color: var(--tertiary);
  }

  .actions {
    display: none;
    gap: 2px;
  }

  .row:hover .actions {
    display: flex;
  }

  .actions button {
    width: 20px;
    height: 20px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--secondary);
  }

  .actions button:hover {
    background: var(--hover);
    color: var(--label);
  }

  .status {
    width: 12px;
    text-align: center;
    font-family: var(--mono);
    font-size: var(--callout);
    font-weight: 600;
  }
</style>
