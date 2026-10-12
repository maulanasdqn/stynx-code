<script>
  import Icon from "../icon.svelte";
  import TreeNode from "./tree-node.svelte";
  import { expanded, ideMessage, ideRoot, tree } from "../../lib/ide/ide-stores.js";
  import { createEntry, loadDir } from "../../lib/ide/file-tree.js";
  import { openFile } from "../../lib/ide/documents.js";

  $: rootName = $ideRoot ? $ideRoot.split("/").filter(Boolean).pop() : "Workspace";
  $: nodes = $tree.get("") ?? [];

  async function newFile() {
    const path = await createEntry("", false);
    if (path) openFile(path);
  }

  function refresh() {
    for (const dir of $tree.keys()) loadDir(dir);
  }

  function collapse() {
    $expanded = new Set([""]);
  }
</script>

<div class="explorer">
  <header class="head">
    <span class="title ellipsis">{rootName}</span>
    <button class="tool" title="New File" on:click={newFile}><Icon name="doc.badge.plus" size={14} /></button>
    <button class="tool" title="New Folder" on:click={() => createEntry("", true)}>
      <Icon name="folder.badge.plus" size={14} />
    </button>
    <button class="tool" title="Refresh" on:click={refresh}><Icon name="arrow.clockwise" size={14} /></button>
    <button class="tool" title="Collapse Folders" on:click={collapse}>
      <Icon name="rectangle.compress.vertical" size={14} />
    </button>
  </header>
  <div class="tree">
    {#each nodes as node (node.path)}
      <TreeNode {node} />
    {:else}
      <div class="empty">{$ideRoot ? "Empty folder" : "Opening workspace…"}</div>
    {/each}
  </div>
  {#if $ideMessage}
    <button class="message ellipsis" title={$ideMessage} on:click={() => ($ideMessage = "")}>{$ideMessage}</button>
  {/if}
</div>

<style>
  .explorer {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .head {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 10px 8px 6px 14px;
  }

  .title {
    flex: 1;
    min-width: 0;
    font-size: var(--subheadline);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: var(--secondary);
  }

  .tool {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--secondary);
  }

  .tool:hover {
    background: var(--hover);
    color: var(--label);
  }

  .tree {
    flex: 1;
    overflow: auto;
    padding: 0 6px 12px;
  }

  .empty {
    padding: 8px 14px;
    font-size: var(--callout);
    color: var(--tertiary);
  }

  .message {
    margin: 6px;
    padding: 6px 10px;
    border-radius: 8px;
    font-size: var(--caption);
    text-align: left;
    color: var(--secondary);
    background: rgba(var(--fg-rgb), 0.06);
  }
</style>
