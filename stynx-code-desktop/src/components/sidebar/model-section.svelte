<script>
  import { info, status, tokens, isStreaming } from "../../lib/stores.js";
  import { applyModel } from "../../lib/session-actions.js";

  export let onOpenWorkspace;

  let draft = "";

  $: if ($info) draft = $info.modelId;
  $: provider = $info?.currentProvider;
  $: baseList =
    provider === "claude" ? $info.claudeModels : provider === "deepseek" ? $info.deepseekModels : null;
  $: presets =
    baseList && $info.modelId && !baseList.includes($info.modelId)
      ? [$info.modelId, ...baseList]
      : baseList;

  function switchProvider(event) {
    onOpenWorkspace($info?.workspacePath ?? null, event.target.value);
  }

  function submit(event) {
    if (event.key === "Enter") applyModel(draft.trim());
  }
</script>

<section>
  <div class="sb-header">Model</div>
  {#if $info}
    <label class="sb-row">
      Main agent
      <select value={provider} on:change={switchProvider} disabled={$isStreaming}>
        {#each $info.mainProviders as option}
          <option value={option}>{option}</option>
        {/each}
      </select>
    </label>
    {#if presets}
      <label class="sb-row">
        Model
        <select value={$info.modelId} on:change={(event) => applyModel(event.target.value)}>
          {#each presets as id}
            <option value={id}>{id}</option>
          {/each}
        </select>
      </label>
    {:else}
      <label class="sb-row">
        Model
        <input
          class="model-id mono"
          bind:value={draft}
          on:keydown={submit}
          placeholder="model id"
          spellcheck="false"
        />
      </label>
    {/if}
  {/if}
  <div class="sb-row">Status <span class="sb-value ellipsis">{$status}</span></div>
  <div class="sb-row">
    Tokens <span class="sb-value">in {$tokens.input} · out {$tokens.output}</span>
  </div>
</section>

<style>
  .model-id {
    flex: 1;
    min-width: 0;
    text-align: right;
    background: none;
    border: none;
    font-size: var(--body);
  }
</style>
