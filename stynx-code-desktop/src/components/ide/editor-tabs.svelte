<script>
  import Icon from "../icon.svelte";
  import { activePath, tabs } from "../../lib/ide/ide-stores.js";
  import { closeTab } from "../../lib/ide/documents.js";

  function middleClick(event, path) {
    if (event.button === 1) closeTab(path);
  }
</script>

<div class="tabs" role="tablist">
  {#each $tabs as tab (tab.path)}
    <div
      class="tab"
      class:active={tab.path === $activePath}
      role="tab"
      tabindex="0"
      aria-selected={tab.path === $activePath}
      title={tab.path}
      on:click={() => ($activePath = tab.path)}
      on:keydown={(event) => event.key === "Enter" && ($activePath = tab.path)}
      on:auxclick={(event) => middleClick(event, tab.path)}
    >
      <span class="name ellipsis" class:conflict={tab.conflict}>{tab.name}</span>
      <button class="close" class:dirty={tab.dirty} title="Close (⌘W)" on:click|stopPropagation={() => closeTab(tab.path)}>
        <span class="dot"><Icon name="circle.fill" size={8} /></span>
        <span class="x"><Icon name="xmark" size={9} weight={2.4} /></span>
      </button>
    </div>
  {/each}
</div>

<style>
  .tabs {
    display: flex;
    gap: 2px;
    padding: 6px 6px 0;
    overflow-x: auto;
    flex-shrink: 0;
    border-bottom: 1px solid var(--separator);
  }

  .tabs::-webkit-scrollbar {
    display: none;
  }

  .tab {
    max-width: 200px;
    height: 30px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 6px 0 12px;
    border-radius: 8px 8px 0 0;
    font-size: var(--callout);
    color: var(--secondary);
    cursor: default;
    transition: background-color 0.2s var(--smooth);
  }

  .tab:hover {
    background: var(--hover);
  }

  .tab.active {
    color: var(--label);
    background: rgba(var(--fg-rgb), 0.07);
    box-shadow: inset 0 -2px 0 var(--accent);
  }

  .name.conflict {
    color: var(--orange);
  }

  .close {
    width: 18px;
    height: 18px;
    border-radius: 5px;
    display: grid;
    place-items: center;
    color: var(--secondary);
  }

  .close > span {
    grid-area: 1 / 1;
    display: flex;
  }

  .close:hover {
    background: var(--hover);
  }

  .dot,
  .close.dirty .x {
    opacity: 0;
  }

  .close.dirty .dot {
    opacity: 1;
  }

  .close.dirty:hover .dot {
    opacity: 0;
  }

  .close.dirty:hover .x,
  .tab:hover .close:not(.dirty) .x,
  .tab.active .close:not(.dirty) .x {
    opacity: 1;
  }

  .x {
    opacity: 0;
  }
</style>
