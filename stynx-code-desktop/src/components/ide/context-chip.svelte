<script>
  import Icon from "../icon.svelte";
  import { editorContext, includeContext } from "../../lib/ide/ide-stores.js";
  import { agentContext, buildContext } from "../../lib/ide/agent-context.js";

  $: available = buildContext($editorContext);
</script>

{#if $agentContext}
  <div class="chips">
    <span class="chip" title="Stynx will see this {available?.hasSelection ? 'selection' : 'file'}">
      <Icon name={$agentContext.hasSelection ? "text.cursor" : "doc.text"} size={11} />
      <span class="label ellipsis">{$agentContext.label}</span>
      <button class="x" title="Don't include" on:click={() => ($includeContext = false)}><Icon name="xmark" size={8} weight={2.6} /></button>
    </span>
  </div>
{:else if available}
  <div class="chips">
    <button class="chip ghost" on:click={() => ($includeContext = true)}>
      <Icon name="plus" size={10} weight={2.4} />
      <span class="label ellipsis">Add {available.label}</span>
    </button>
  </div>
{/if}

<style>
  .chips { display: flex; gap: 6px; }
  .chip {
    max-width: 100%;
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 3px 4px 3px 8px;
    border-radius: 999px;
    font-size: var(--caption);
    color: var(--accent);
    background: rgba(var(--accent-rgb), 0.13);
    border: 1px solid rgba(var(--accent-rgb), 0.25);
  }
  .chip.ghost { padding-right: 9px; color: var(--secondary); background: none; border-style: dashed; border-color: var(--separator); }
  .chip.ghost:hover { color: var(--accent); border-color: rgba(var(--accent-rgb), 0.4); }
  .label { min-width: 0; font-family: var(--mono); }
  .x { display: flex; padding: 3px; border-radius: 999px; color: inherit; }
  .x:hover { background: rgba(var(--accent-rgb), 0.2); }
</style>
