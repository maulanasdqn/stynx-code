<script>
  import Icon from "../icon.svelte";
  import { activePath, expanded, tree } from "../../lib/ide/ide-stores.js";
  import { createEntry, deleteEntry, renameEntry, toggleDir } from "../../lib/ide/file-tree.js";
  import { openFile, retargetTab, closeTab } from "../../lib/ide/documents.js";
  import { popupMenu } from "../../lib/native.js";
  import { gitBadges } from "../../lib/ide/tree-badges.js";

  export let node;
  export let depth = 0;

  $: open = node.isDir && $expanded.has(node.path);
  $: children = open ? $tree.get(node.path) ?? [] : [];
  $: badge = $gitBadges.get(node.path);
  $: tone = badge === "?" || badge === "A" ? "added" : badge === "D" || badge === "U" ? "removed" : badge ? "modified" : "";

  function activate() {
    if (node.isDir) toggleDir(node.path);
    else openFile(node.path);
  }

  async function create(isDir) {
    const dir = node.isDir ? node.path : node.path.slice(0, node.path.lastIndexOf("/"));
    const path = await createEntry(dir, isDir);
    if (path) openFile(path);
  }

  async function rename() {
    const target = await renameEntry(node.path);
    if (target) retargetTab(node.path, target);
  }

  async function remove() {
    if (await deleteEntry(node.path)) closeTab(node.path);
  }

  function contextMenu(event) {
    event.preventDefault();
    popupMenu(
      [
        { text: "New File", action: () => create(false) },
        { text: "New Folder", action: () => create(true) },
        "-",
        { text: "Rename…", action: rename },
        { text: "Delete", action: remove },
      ],
      event,
    );
  }
</script>

<button
  class="row"
  class:active={!node.isDir && $activePath === node.path}
  style="padding-left: {10 + depth * 14}px"
  title={node.path}
  on:click={activate}
  on:contextmenu={contextMenu}
>
  <span class="chevron" class:open class:hidden={!node.isDir}>
    <Icon name="chevron.right" size={9} weight={2.4} />
  </span>
  <span class="glyph" class:folder={node.isDir}>
    <Icon name={node.isDir ? (open ? "folder.fill" : "folder") : "doc"} size={13} />
  </span>
  <span class="name ellipsis {tone}">{node.name}</span>
  {#if badge}<span class="badge {tone}">{badge === "?" ? "U" : badge}</span>{/if}
</button>
{#if open}
  {#each children as child (child.path)}
    <svelte:self node={child} depth={depth + 1} />
  {/each}
{/if}

<style>
  .row {
    width: 100%;
    height: 24px;
    display: flex;
    align-items: center;
    gap: 5px;
    padding-right: 10px;
    border-radius: 6px;
    text-align: left;
    font-size: var(--body);
    color: var(--label);
  }

  .row:hover {
    background: var(--hover);
  }

  .row.active {
    background: rgba(var(--accent-rgb), 0.16);
  }

  .chevron {
    width: 10px;
    display: flex;
    color: var(--secondary);
    transition: transform 0.35s var(--spring);
  }

  .chevron.open {
    transform: rotate(90deg);
  }

  .chevron.hidden {
    visibility: hidden;
  }

  .glyph {
    display: flex;
    color: var(--secondary);
  }

  .glyph.folder {
    color: var(--accent);
  }

  .name {
    flex: 1;
    min-width: 0;
  }

  .badge {
    font-family: var(--mono);
    font-size: var(--caption);
    font-weight: 600;
  }

  .added {
    color: var(--green);
  }

  .modified {
    color: var(--orange);
  }

  .removed {
    color: var(--red);
  }
</style>
