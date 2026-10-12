<script>
  import { onMount, tick } from "svelte";
  import Icon from "../icon.svelte";
  import Sheet from "../sheet.svelte";
  import { fileIndex, quickOpenVisible } from "../../lib/ide/ide-stores.js";
  import { fuzzyFilter } from "../../lib/ide/fuzzy.js";
  import { ensureIndex } from "../../lib/ide/workspace-session.js";
  import { openFile } from "../../lib/ide/documents.js";

  let query = "";
  let selected = 0;
  let input;
  let list;

  $: results = fuzzyFilter(query, $fileIndex);
  $: query, (selected = 0);

  onMount(() => {
    input.focus();
    ensureIndex();
  });

  const close = () => ($quickOpenVisible = false);

  function choose(path) {
    if (!path) return;
    close();
    openFile(path);
  }

  async function keydown(event) {
    if (event.key === "ArrowDown" || event.key === "ArrowUp") {
      event.preventDefault();
      const step = event.key === "ArrowDown" ? 1 : -1;
      selected = Math.max(0, Math.min(results.length - 1, selected + step));
      await tick();
      list?.querySelector(".selected")?.scrollIntoView({ block: "nearest" });
    } else if (event.key === "Enter") {
      event.preventDefault();
      choose(results[selected]);
    }
  }

  const split = (path) => {
    const at = path.lastIndexOf("/");
    return at < 0 ? ["", path] : [path.slice(0, at), path.slice(at + 1)];
  };
</script>

<Sheet width={560} onCancel={close}>
  <div class="quick">
    <div class="search">
      <Icon name="magnifyingglass" size={14} />
      <input bind:this={input} bind:value={query} on:keydown={keydown} placeholder="Go to file…" spellcheck="false" />
    </div>
    <div class="results" bind:this={list}>
      {#each results as path, index (path)}
        {@const [dir, name] = split(path)}
        <button class="result" class:selected={index === selected} on:click={() => choose(path)} on:mousemove={() => (selected = index)}>
          <Icon name="doc" size={13} />
          <span class="name">{name}</span>
          <span class="dir ellipsis">{dir}</span>
        </button>
      {:else}
        <div class="empty">{$fileIndex.length ? "No matching files" : "Indexing…"}</div>
      {/each}
    </div>
  </div>
</Sheet>

<style>
  .quick {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: -8px;
  }

  .search {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 10px;
    color: var(--secondary);
    background: rgba(var(--fg-rgb), 0.06);
  }

  input {
    flex: 1;
    background: none;
    border: none;
    font-size: var(--title3);
  }

  .results {
    max-height: 360px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .result {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    border-radius: 8px;
    text-align: left;
    color: var(--secondary);
  }

  .result.selected {
    background: rgba(var(--accent-rgb), 0.9);
    color: white;
  }

  .name {
    color: var(--label);
    white-space: nowrap;
  }

  .result.selected .name,
  .result.selected .dir {
    color: white;
  }

  .dir {
    min-width: 0;
    font-size: var(--callout);
    color: var(--tertiary);
  }

  .empty {
    padding: 12px;
    font-size: var(--callout);
    color: var(--tertiary);
  }
</style>
