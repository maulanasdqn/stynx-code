<script>
  import { onMount } from "svelte";
  import Icon from "../icon.svelte";
  import { openMatch, runSearch, searchResults, searchState } from "../../lib/ide/search.js";

  let input;

  onMount(() => input?.focus());

  $: runSearch($searchState);
  $: count = $searchResults.files.reduce((sum, file) => sum + file.matches.length, 0);

  function parts(match) {
    const chars = [...match.preview];
    const lead = chars.length - [...match.preview.trimStart()].length;
    const start = Math.max(0, match.offset - lead);
    const text = chars.slice(lead);
    return [text.slice(0, start).join(""), text.slice(start, start + match.length).join(""), text.slice(start + match.length).join("")];
  }
</script>

<div class="search">
  <header class="head"><span class="title">Search</span></header>
  <div class="box">
    <input bind:this={input} bind:value={$searchState.query} placeholder="Search in files" spellcheck="false" />
    <button class="toggle" class:on={$searchState.caseSensitive} title="Match Case" on:click={() => ($searchState.caseSensitive = !$searchState.caseSensitive)}>Aa</button>
    <button class="toggle" class:on={$searchState.regex} title="Use Regular Expression" on:click={() => ($searchState.regex = !$searchState.regex)}>.*</button>
  </div>
  <div class="status">
    {#if $searchResults.busy}<span class="spinner small"></span> Searching…
    {:else if $searchResults.error}<span class="error">{$searchResults.error}</span>
    {:else if $searchState.query}{count} result{count === 1 ? "" : "s"} in {$searchResults.files.length} file{$searchResults.files.length === 1 ? "" : "s"}{$searchResults.truncated ? " (limited)" : ""}
    {/if}
  </div>
  <div class="list">
    {#each $searchResults.files as file (file.path)}
      <div class="file">
        <Icon name="doc" size={12} />
        <span class="name">{file.path.split("/").pop()}</span>
        <span class="dir ellipsis">{file.path}</span>
        <span class="count">{file.matches.length}</span>
      </div>
      {#each file.matches as match, index (index)}
        {@const [before, hit, after] = parts(match)}
        <button class="match" on:click={() => openMatch(file.path, match)}>
          <span class="text ellipsis">{before}<mark>{hit}</mark>{after}</span>
          <span class="line">{match.line}</span>
        </button>
      {/each}
    {/each}
  </div>
</div>

<style>
  .search { flex: 1; min-height: 0; display: flex; flex-direction: column; }
  .head { padding: 10px 8px 6px 14px; }
  .title { font-size: var(--subheadline); font-weight: 700; text-transform: uppercase; letter-spacing: 0.4px; color: var(--secondary); }
  .box { display: flex; align-items: center; gap: 2px; margin: 0 10px; padding: 3px 4px 3px 9px; border-radius: 8px; background: rgba(var(--fg-rgb), 0.06); }
  .box input { flex: 1; min-width: 0; background: none; border: none; font-size: var(--callout); }
  .toggle { width: 24px; height: 22px; border-radius: 6px; font-family: var(--mono); font-size: var(--caption); color: var(--secondary); }
  .toggle.on { color: white; background: var(--accent); }
  .status { display: flex; align-items: center; gap: 6px; min-height: 22px; padding: 4px 14px; font-size: var(--caption); color: var(--tertiary); }
  .error { color: var(--red); }
  .list { flex: 1; overflow: auto; padding: 0 6px 12px; }
  .file { display: flex; align-items: center; gap: 6px; padding: 8px 8px 3px; color: var(--secondary); }
  .name { color: var(--label); font-size: var(--body); white-space: nowrap; }
  .dir { flex: 1; min-width: 0; font-size: var(--caption); color: var(--tertiary); }
  .count { font-size: var(--caption); color: var(--tertiary); }
  .match { width: 100%; display: flex; align-items: center; gap: 6px; padding: 3px 8px 3px 26px; border-radius: 6px; text-align: left; }
  .match:hover { background: var(--hover); }
  .text { flex: 1; min-width: 0; font-family: var(--mono); font-size: var(--caption); color: var(--secondary); white-space: pre; }
  mark { color: var(--label); background: rgba(var(--accent-rgb), 0.3); border-radius: 3px; }
  .line { font-family: var(--mono); font-size: 10px; color: var(--tertiary); }
  .spinner.small { width: 10px; height: 10px; }
</style>
