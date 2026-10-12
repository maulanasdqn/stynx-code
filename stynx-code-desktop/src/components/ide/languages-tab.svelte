<script>
  import { catalog } from "../../lib/ide/languages.js";
  import { autoInstall, installedLanguages, installLanguage, uninstallLanguage } from "../../lib/ide/syntax.js";

  export let query = "";

  let busy = new Set();

  $: needle = query.trim().toLowerCase();
  $: list = catalog
    .filter((lang) => !needle || lang.name.toLowerCase().includes(needle) || lang.id.includes(needle) || lang.aliases.some((a) => a.includes(needle)))
    .sort((a, b) => Number($installedLanguages.has(b.id)) - Number($installedLanguages.has(a.id)));

  async function install(id) {
    busy = new Set(busy).add(id);
    await installLanguage(id);
    busy.delete(id);
    busy = new Set(busy);
  }
</script>

<label class="auto">
  <input type="checkbox" bind:checked={$autoInstall} />
  Install syntax automatically when opening a file
</label>
<div class="summary">{$installedLanguages.size} of {catalog.length} languages installed</div>
{#each list as lang (lang.id)}
  <div class="row">
    <span class="meta">
      <span class="name">{lang.name}</span>
      <span class="alias ellipsis">{[lang.id, ...lang.aliases].join(", ")}</span>
    </span>
    {#if $installedLanguages.has(lang.id)}
      <button class="push" on:click={() => uninstallLanguage(lang.id)}>Remove</button>
    {:else}
      <button class="push prominent" disabled={busy.has(lang.id)} on:click={() => install(lang.id)}>
        {busy.has(lang.id) ? "Installing…" : "Install"}
      </button>
    {/if}
  </div>
{/each}

<style>
  .auto { display: flex; align-items: center; gap: 8px; padding: 4px 8px 6px; font-size: var(--callout); }
  .auto input { accent-color: var(--accent); }
  .summary { padding: 0 8px 8px; font-size: var(--caption); color: var(--tertiary); }
  .row { display: flex; align-items: center; gap: 8px; padding: 6px 8px; border-radius: 8px; }
  .row:hover { background: var(--hover); }
  .meta { flex: 1; min-width: 0; display: flex; flex-direction: column; }
  .name { font-size: var(--body); }
  .alias { font-size: var(--caption); color: var(--tertiary); font-family: var(--mono); }
  .push { font-size: var(--callout); padding: 2px 10px; }
</style>
