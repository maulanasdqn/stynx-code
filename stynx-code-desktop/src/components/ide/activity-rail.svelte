<script>
  import Icon from "../icon.svelte";
  import { activeSidePanel, agentVisible, diagnostics, gitState, terminalVisible } from "../../lib/ide/ide-stores.js";
  import { toggleTerminalPanel } from "../../lib/ide/terminals.js";

  const PANELS = [
    { id: "explorer", icon: "folder.stack", label: "Explorer" },
    { id: "search", icon: "magnifyingglass", label: "Search" },
    { id: "git", icon: "arrow.triangle.branch", label: "Source Control" },
    { id: "problems", icon: "exclamationmark.triangle.fill", label: "Problems" },
    { id: "interns", icon: "person.2", label: "Interns" },
    { id: "extensions", icon: "puzzlepiece.extension", label: "Extensions" },
  ];

  $: changes = $gitState.staged.length + $gitState.unstaged.length + $gitState.conflicted.length;
  $: errors = [...$diagnostics.values()].flat().filter((d) => (d.severity ?? 1) === 1).length;
  $: counts = { git: changes, problems: errors };

  function select(id) {
    $activeSidePanel = $activeSidePanel === id ? null : id;
  }
</script>

<nav class="rail">
  {#each PANELS as panel}
    <button class="item" class:active={$activeSidePanel === panel.id} title={panel.label} on:click={() => select(panel.id)}>
      <Icon name={panel.icon} size={17} weight={1.6} />
      {#if counts[panel.id] > 0}<span class="badge" class:alert={panel.id === "problems"}>{counts[panel.id] > 99 ? "99+" : counts[panel.id]}</span>{/if}
    </button>
  {/each}
  <button class="item" class:active={$terminalVisible} title="Terminal (⌘`)" on:click={toggleTerminalPanel}>
    <Icon name="terminal" size={17} weight={1.6} />
  </button>
  <span class="spacer"></span>
  <button class="item agent" class:active={$agentVisible} title="Stynx Agent (⌘L)" on:click={() => ($agentVisible = !$agentVisible)}>
    <Icon name="sparkles" size={16} />
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

  .badge.alert {
    background: var(--red);
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
