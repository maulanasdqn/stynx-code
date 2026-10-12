<script>
  import { onMount } from "svelte";
  import Icon from "../icon.svelte";
  import InternSheet from "./intern-sheet.svelte";
  import { deleteIntern, internEditor, internRows, refreshInterns } from "../../lib/ide/interns.js";
  import { mcpPendingReload, reloadAgent } from "../../lib/ide/mcp.js";

  onMount(refreshInterns);

  const blank = () => ({ name: "", provider: "deepseek", model: "", description: "", baseUrl: "", apiKeyEnv: "" });

  function edit(row) {
    $internEditor = row.config
      ? { entry: { ...blank(), ...row.config }, previous: row.config.name }
      : { entry: { ...blank(), name: row.name, provider: row.provider, model: row.model, description: row.description, apiKeyEnv: row.keyEnv }, previous: null };
  }
</script>

<div class="interns">
  <header class="head">
    <span class="title">Interns</span>
    <button class="tool" title="Add Intern" on:click={() => ($internEditor = { entry: blank(), previous: null })}><Icon name="plus" size={14} weight={2} /></button>
    <button class="tool" title="Refresh" on:click={refreshInterns}><Icon name="arrow.clockwise" size={14} /></button>
  </header>
  <p class="intro">Interns are cheaper models the agent can delegate tasks to. They're saved in ~/.stynx/settings.json.</p>
  {#if $mcpPendingReload}
    <div class="reload">
      <span>Reload the agent to apply changes. This starts a new conversation.</span>
      <button class="push prominent" on:click={reloadAgent}>Reload Agent</button>
    </div>
  {/if}
  <div class="list">
    {#each $internRows as row (row.name)}
      <div class="card">
        <div class="top">
          <span class="dot" class:ready={row.available}></span>
          <span class="name mono">{row.name}</span>
          <span class="state">{row.available ? "ready" : row.keyEnv ? `needs ${row.keyEnv}` : "not configured"}</span>
          <span class="spacer"></span>
          <button class="tool" title="Edit" on:click={() => edit(row)}><Icon name="pencil" size={12} /></button>
          {#if row.config}
            <button class="tool" title="Remove" on:click={() => deleteIntern(row.name)}><Icon name="xmark" size={10} weight={2.4} /></button>
          {/if}
        </div>
        <div class="model mono ellipsis">{row.provider} · {row.model}</div>
        {#if row.description}<div class="desc">{row.description}</div>{/if}
      </div>
    {:else}
      <div class="empty">No interns yet. Add one to let the agent delegate work.</div>
    {/each}
  </div>
</div>
{#if $internEditor}
  <InternSheet editor={$internEditor} />
{/if}

<style>
  .interns { flex: 1; min-height: 0; display: flex; flex-direction: column; }
  .head { display: flex; align-items: center; gap: 2px; padding: 10px 8px 4px 14px; }
  .title { flex: 1; font-size: var(--subheadline); font-weight: 700; text-transform: uppercase; letter-spacing: 0.4px; color: var(--secondary); }
  .intro { padding: 0 14px 8px; font-size: var(--caption); color: var(--tertiary); }
  .reload { margin: 0 10px 8px; padding: 8px 10px; border-radius: 10px; display: flex; flex-direction: column; gap: 6px; font-size: var(--caption); background: rgba(var(--accent-rgb), 0.12); }
  .list { flex: 1; overflow: auto; padding: 0 6px 12px; }
  .card { padding: 8px; border-radius: 10px; display: flex; flex-direction: column; gap: 3px; }
  .card:hover { background: var(--hover); }
  .top { display: flex; align-items: center; gap: 7px; }
  .dot { width: 7px; height: 7px; border-radius: 50%; background: var(--orange); flex-shrink: 0; }
  .dot.ready { background: var(--green); }
  .name { font-size: var(--body); font-weight: 600; }
  .state { font-size: var(--caption); color: var(--secondary); }
  .spacer { flex: 1; }
  .model { padding-left: 14px; font-size: var(--caption); color: var(--secondary); }
  .desc { padding-left: 14px; font-size: var(--caption); color: var(--tertiary); }
  .tool { width: 24px; height: 24px; border-radius: 6px; display: flex; align-items: center; justify-content: center; color: var(--secondary); }
  .tool:hover { background: var(--hover); color: var(--label); }
  .push { font-size: var(--callout); padding: 2px 10px; }
  .empty { padding: 10px 8px; font-size: var(--callout); color: var(--tertiary); }
</style>
