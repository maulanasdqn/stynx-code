<script>
  import Icon from "../icon.svelte";
  import TerminalView from "./terminal-view.svelte";
  import { activeTerminal, terminals, terminalVisible } from "../../lib/ide/ide-stores.js";
  import { closeTerminal, newTerminal } from "../../lib/ide/terminals.js";
</script>

<div class="panel">
  <header class="bar">
    <span class="title">Terminal</span>
    <div class="tabs">
      {#each $terminals as terminal (terminal.key)}
        <button
          class="tab"
          class:active={terminal.key === $activeTerminal}
          class:exited={terminal.exited}
          on:click={() => ($activeTerminal = terminal.key)}
        >
          <Icon name="terminal" size={11} />
          {terminal.label}
          <span class="close" role="button" tabindex="-1" on:click|stopPropagation={() => closeTerminal(terminal.key)} on:keydown={() => {}}>
            <Icon name="xmark" size={8} weight={2.6} />
          </span>
        </button>
      {/each}
    </div>
    <span class="spacer"></span>
    <button class="tool" title="New Terminal" on:click={() => newTerminal()}><Icon name="plus" size={14} weight={2} /></button>
    <button class="tool" title="Hide Panel (⌘`)" on:click={() => ($terminalVisible = false)}>
      <Icon name="chevron.down" size={11} weight={2.2} />
    </button>
  </header>
  <div class="views">
    {#each $terminals as terminal (terminal.key)}
      <TerminalView key={terminal.key} active={terminal.key === $activeTerminal} />
    {/each}
  </div>
</div>

<style>
  .panel {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .bar {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 34px;
    padding: 0 8px 0 14px;
    flex-shrink: 0;
  }

  .title {
    font-size: var(--subheadline);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: var(--secondary);
  }

  .tabs {
    display: flex;
    gap: 4px;
    min-width: 0;
    overflow-x: auto;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 5px;
    height: 24px;
    padding: 0 6px 0 9px;
    border-radius: 999px;
    font-size: var(--caption);
    white-space: nowrap;
    color: var(--secondary);
  }

  .tab:hover {
    background: var(--hover);
  }

  .tab.active {
    color: var(--label);
    background: rgba(var(--fg-rgb), 0.08);
  }

  .tab.exited {
    opacity: 0.55;
  }

  .close {
    display: flex;
    padding: 3px;
    border-radius: 999px;
  }

  .close:hover {
    background: var(--hover);
  }

  .spacer {
    flex: 1;
  }

  .tool {
    width: 26px;
    height: 26px;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--secondary);
  }

  .tool:hover {
    background: var(--hover);
    color: var(--label);
  }

  .views {
    flex: 1;
    min-height: 0;
    position: relative;
  }
</style>
