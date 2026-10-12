<script>
  import Icon from "../icon.svelte";
  import { activeSidePanel, gitState, quickOpenVisible, terminalVisible } from "../../lib/ide/ide-stores.js";
  import { toggleTerminalPanel } from "../../lib/ide/terminals.js";

  export let showAgent = true;

  const PANELS = [
    { id: "explorer", icon: "folder.stack", label: "Explorer" },
    { id: "git", icon: "arrow.triangle.branch", label: "Source Control" },
  ];

  $: changes = $gitState.staged.length + $gitState.unstaged.length + $gitState.conflicted.length;

  function select(id) {
    $activeSidePanel = $activeSidePanel === id ? null : id;
  }
</script>

<nav class="rail">
  {#each PANELS as panel}
    <button class="item" class:active={$activeSidePanel === panel.id} title={panel.label} on:click={() => select(panel.id)}>
      <Icon name={panel.icon} size={17} weight={1.6} />
      {#if panel.id === "git" && changes > 0}<span class="badge">{changes > 99 ? "99+" : changes}</span>{/if}
    </button>
  {/each}
  <button class="item" title="Go to File (⌘P)" on:click={() => ($quickOpenVisible = true)}>
    <Icon name="magnifyingglass" size={17} weight={1.6} />
  </button>
  <button class="item" class:active={$terminalVisible} title="Terminal (⌘`)" on:click={toggleTerminalPanel}>
    <Icon name="terminal" size={17} weight={1.6} />
  </button>
  <span class="spacer"></span>
  <button class="item" class:active={showAgent} title="Agent" on:click={() => (showAgent = !showAgent)}>
    <Icon name="bubble.left.and.text.bubble.right" size={17} weight={1.6} />
  </button>
</nav>

<style>
  .rail {
    width: 44px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 6px 0;
  }

  .spacer {
    flex: 1;
  }

  .item {
    width: 34px;
    height: 34px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--secondary);
  }

  .item {
    position: relative;
  }

  .badge {
    position: absolute;
    top: 2px;
    right: 1px;
    min-width: 15px;
    height: 15px;
    padding: 0 4px;
    border-radius: 999px;
    font-size: 9px;
    font-weight: 700;
    line-height: 15px;
    color: white;
    background: var(--accent);
  }

  .item:hover {
    background: var(--hover);
    color: var(--label);
  }

  .item.active {
    color: var(--accent);
    background: rgba(var(--accent-rgb), 0.12);
  }
</style>
