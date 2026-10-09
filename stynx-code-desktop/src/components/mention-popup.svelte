<script>
  import Icon from "./icon.svelte";
  import { composerDraft } from "../lib/stores.js";
  import { currentMention, mentionSuggestions, applyMention } from "../lib/mentions.js";

  $: query = currentMention($composerDraft);
  $: suggestions = query === null ? [] : mentionSuggestions(query);

  function pick(path) {
    $composerDraft = applyMention($composerDraft, path);
  }
</script>

{#if suggestions.length > 0}
  <div class="popup glass panel">
    {#each suggestions as path (path)}
      <button on:click={() => pick(path)}>
        <span class="icon"><Icon name="doc" size={11} /></span>
        <span class="path mono ellipsis">{path}</span>
      </button>
    {/each}
  </div>
{/if}

<style>
  .popup {
    display: flex;
    flex-direction: column;
    padding: 6px;
    border-radius: 14px;
    max-height: 240px;
    overflow-y: auto;
    box-shadow: none;
  }

  button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    border-radius: 8px;
    text-align: left;
  }

  button:hover {
    background: var(--hover);
  }

  .icon {
    color: var(--secondary);
  }

  .path {
    font-size: var(--callout);
  }
</style>
