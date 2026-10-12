<script>
  import { onMount } from "svelte";
  import { installLogs } from "../../lib/ide/ide-stores.js";
  import { installServer, refreshServers, serverBusy, servers, uninstallServer } from "../../lib/ide/servers.js";

  export let query = "";

  onMount(refreshServers);

  $: needle = query.trim().toLowerCase();
  $: list = $servers.filter((s) => !needle || s.name.toLowerCase().includes(needle) || s.extensions.some((e) => e.includes(needle)));
</script>

{#each list as server (server.id)}
  {@const busy = $serverBusy.has(server.id)}
  <div class="card">
    <div class="top">
      <span class="dot" class:on={server.location}></span>
      <span class="name">{server.name}</span>
      <span class="exts ellipsis">.{server.extensions.join(" .")}</span>
      {#if busy}
        <span class="spinner small"></span>
      {:else if server.managed}
        <button class="push" on:click={() => uninstallServer(server.id)}>Remove</button>
      {:else if !server.location && server.installable}
        <button class="push prominent" on:click={() => installServer(server.id)}>Install</button>
      {/if}
    </div>
    {#if server.location}
      <div class="sub ellipsis" title={server.location}>{server.managed ? "Installed by Stynx" : server.location}</div>
    {:else if !server.installable}
      <div class="sub">Install with <code>{server.hint}</code></div>
    {:else}
      <div class="sub">Not installed — installs into ~/.stynx/lsp with npm</div>
    {/if}
    {#if busy || ($installLogs.get(server.id)?.length && !server.location)}
      <pre class="log">{($installLogs.get(server.id) ?? []).slice(-6).join("\n") || "Starting npm…"}</pre>
    {/if}
  </div>
{/each}

<style>
  .card { padding: 8px 8px 9px; border-radius: 10px; display: flex; flex-direction: column; gap: 3px; }
  .card:hover { background: var(--hover); }
  .top { display: flex; align-items: center; gap: 7px; }
  .dot { width: 7px; height: 7px; border-radius: 50%; background: var(--tertiary); flex-shrink: 0; }
  .dot.on { background: var(--green); }
  .name { font-size: var(--body); font-weight: 600; white-space: nowrap; }
  .exts { flex: 1; min-width: 0; font-size: var(--caption); color: var(--tertiary); font-family: var(--mono); }
  .sub { padding-left: 14px; font-size: var(--caption); color: var(--secondary); }
  code { font-family: var(--mono); color: var(--label); user-select: text; -webkit-user-select: text; }
  .log { margin: 4px 0 0 14px; padding: 6px 8px; border-radius: 6px; max-height: 96px; overflow: auto; font-family: var(--mono); font-size: 10px; color: var(--secondary); background: rgba(var(--fg-rgb), 0.05); white-space: pre-wrap; }
  .push { font-size: var(--callout); padding: 2px 10px; }
  .spinner.small { width: 12px; height: 12px; }
</style>
