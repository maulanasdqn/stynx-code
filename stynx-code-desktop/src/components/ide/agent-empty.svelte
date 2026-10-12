<script>
  import Icon from "../icon.svelte";
  import { editorContext } from "../../lib/ide/ide-stores.js";
  import { askAgent, suggestionsFor } from "../../lib/ide/agent-context.js";

  $: name = $editorContext?.path?.split("/").pop();
  $: suggestions = suggestionsFor($editorContext);
</script>

<div class="empty">
  <span class="mark"><Icon name="sparkles" size={22} /></span>
  <div class="headline">Ask Stynx about your code</div>
  <div class="sub">
    {#if name}It can see <span class="mono">{name}</span>. Select code and press <kbd>⌘L</kbd> to ask about just that part.
    {:else}Open a file or select code, then press <kbd>⌘L</kbd> to ask about it.{/if}
  </div>
  <div class="suggestions">
    {#each suggestions as text}
      <button class="suggestion" on:click={() => askAgent(text)}>
        <span>{text}</span>
        <Icon name="arrow.right" size={10} weight={2.2} />
      </button>
    {/each}
  </div>
</div>

<style>
  .empty { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 8px; padding: 24px 18px; text-align: center; }
  .mark { width: 44px; height: 44px; border-radius: 14px; display: flex; align-items: center; justify-content: center; color: white; background: linear-gradient(145deg, rgba(var(--accent-rgb), 1), rgba(var(--accent-rgb), 0.65)); box-shadow: 0 6px 18px rgba(var(--accent-rgb), 0.35); }
  .headline { margin-top: 4px; font-size: var(--title3); font-weight: 600; }
  .sub { max-width: 280px; font-size: var(--callout); color: var(--secondary); line-height: 1.45; }
  kbd { font-family: var(--mono); font-size: var(--caption); padding: 1px 5px; border-radius: 5px; background: rgba(var(--fg-rgb), 0.08); border: 1px solid var(--separator); }
  .suggestions { width: 100%; max-width: 300px; margin-top: 10px; display: flex; flex-direction: column; gap: 6px; }
  .suggestion { display: flex; align-items: center; justify-content: space-between; gap: 8px; padding: 8px 12px; border-radius: 10px; text-align: left; font-size: var(--callout); color: var(--label); background: rgba(var(--fg-rgb), 0.05); border: 1px solid var(--separator); }
  .suggestion:hover { border-color: rgba(var(--accent-rgb), 0.45); background: rgba(var(--accent-rgb), 0.08); }
</style>
