<script>
  import Icon from "../icon.svelte";
  import LanguagesTab from "./languages-tab.svelte";

  const TABS = [{ id: "languages", label: "Languages" }];

  let tab = "languages";
  let query = "";
</script>

<div class="extensions">
  <header class="head"><span class="title">Extensions</span></header>
  <div class="tabs">
    {#each TABS as entry}
      <button class="tab" class:active={tab === entry.id} on:click={() => (tab = entry.id)}>{entry.label}</button>
    {/each}
  </div>
  <div class="search">
    <Icon name="magnifyingglass" size={12} />
    <input bind:value={query} placeholder="Search {TABS.find((t) => t.id === tab).label.toLowerCase()}…" spellcheck="false" />
  </div>
  <div class="list">
    {#if tab === "languages"}<LanguagesTab {query} />{/if}
  </div>
</div>

<style>
  .extensions { flex: 1; min-height: 0; display: flex; flex-direction: column; }
  .head { padding: 10px 8px 6px 14px; }
  .title { font-size: var(--subheadline); font-weight: 700; text-transform: uppercase; letter-spacing: 0.4px; color: var(--secondary); }
  .tabs { display: flex; gap: 4px; padding: 0 10px 8px; }
  .tab { flex: 1; padding: 4px 6px; border-radius: 7px; font-size: var(--caption); font-weight: 600; color: var(--secondary); }
  .tab:hover { background: var(--hover); }
  .tab.active { color: var(--label); background: rgba(var(--fg-rgb), 0.09); }
  .search { display: flex; align-items: center; gap: 6px; margin: 0 10px 8px; padding: 5px 9px; border-radius: 8px; color: var(--secondary); background: rgba(var(--fg-rgb), 0.06); }
  .search input { flex: 1; min-width: 0; background: none; border: none; font-size: var(--callout); }
  .list { flex: 1; overflow: auto; padding: 0 6px 12px; }
</style>
