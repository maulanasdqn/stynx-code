<script>
  import Icon from "../icon.svelte";
  import Chat from "../chat.svelte";
  import AgentEmpty from "./agent-empty.svelte";
  import ContextChip from "./context-chip.svelte";
  import { info, isStreaming, status } from "../../lib/stores.js";
  import { agentVisible, viewMode } from "../../lib/ide/ide-stores.js";
  import { agentContext } from "../../lib/ide/agent-context.js";
  import { startNewSession } from "../../lib/session-actions.js";

  export let visible = true;

  $: model = ($info?.modelId ?? "").replace(/^claude-/, "").replace("[1m]", " · 1M");
  $: placeholder = $agentContext ? `Ask Stynx about ${$agentContext.label}…` : "Ask Stynx anything about this project…";
</script>

<section class="agent">
  <header class="head">
    <span class="mark"><Icon name="sparkles" size={13} /></span>
    <span class="titles">
      <span class="title">Stynx Agent</span>
      <span class="model ellipsis">{model || "Starting…"}</span>
    </span>
    <span class="status" class:busy={$isStreaming}>
      {#if $isStreaming}<span class="spinner small"></span>{:else}<span class="dot"></span>{/if}
      <span class="ellipsis">{$isStreaming ? $status : "Ready"}</span>
    </span>
    <button class="tool" title="New conversation" disabled={$isStreaming} on:click={startNewSession}><Icon name="square.and.pencil" size={14} /></button>
    <button class="tool" title="Open in Chat mode" on:click={() => ($viewMode = "chat")}><Icon name="arrow.up.left.and.arrow.down.right" size={13} /></button>
    <button class="tool" title="Hide agent" on:click={() => ($agentVisible = false)}><Icon name="xmark" size={10} weight={2.4} /></button>
  </header>
  <div class="body">
    {#if visible}
      <Chat {placeholder} context={$agentContext}>
        <svelte:fragment slot="empty"><AgentEmpty /></svelte:fragment>
        <svelte:fragment slot="composer-above"><ContextChip /></svelte:fragment>
      </Chat>
    {/if}
  </div>
</section>

<style>
  .agent {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    background: var(--agent-surface);
  }
  .head { display: flex; align-items: center; gap: 8px; padding: 10px 8px 10px 12px; background: var(--agent-header); border-bottom: 1px solid var(--agent-edge); }
  .mark { width: 26px; height: 26px; border-radius: 8px; flex-shrink: 0; display: flex; align-items: center; justify-content: center; color: white; background: linear-gradient(145deg, rgba(var(--accent-rgb), 1), rgba(var(--accent-rgb), 0.65)); }
  .titles { display: flex; flex-direction: column; min-width: 0; line-height: 1.2; }
  .title { font-size: var(--headline); font-weight: 700; }
  .model { font-size: var(--caption); color: var(--secondary); font-family: var(--mono); }
  .status { flex: 1; min-width: 0; display: flex; align-items: center; justify-content: flex-end; gap: 5px; font-size: var(--caption); color: var(--secondary); }
  .status.busy { color: var(--accent); }
  .dot { width: 6px; height: 6px; border-radius: 50%; background: var(--green); flex-shrink: 0; }
  .tool { width: 26px; height: 26px; flex-shrink: 0; border-radius: 7px; display: flex; align-items: center; justify-content: center; color: var(--secondary); }
  .tool:hover:not(:disabled) { background: var(--hover); color: var(--label); }
  .body { flex: 1; min-height: 0; display: flex; position: relative; --toolbar-h: 0px; }
  .spinner.small { width: 10px; height: 10px; }
</style>
