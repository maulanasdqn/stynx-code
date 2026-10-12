<script>
  import { onMount } from "svelte";
  import Icon from "../icon.svelte";
  import McpInstallSheet from "./mcp-install-sheet.svelte";
  import { openExternal } from "../../lib/native.js";
  import { loadMore, mcpCursor, mcpInstallTarget, mcpLoading, mcpPendingReload, mcpResults, mcpServersInstalled, refreshInstalled, reloadAgent, searchRegistry, uninstall } from "../../lib/ide/mcp.js";

  export let query = "";

  onMount(refreshInstalled);
  $: searchRegistry(query);

  const KIND = { npm: "npm", pypi: "PyPI", oci: "Docker", remote: "Remote" };
</script>

{#if $mcpPendingReload}
  <div class="reload">
    <span>Reload the agent to load MCP changes. This starts a new conversation.</span>
    <button class="push prominent" on:click={reloadAgent}>Reload Agent</button>
  </div>
{/if}
{#if $mcpServersInstalled.length}
  <div class="section">Installed</div>
  {#each $mcpServersInstalled as item (item.scope + item.name)}
    <div class="row">
      <span class="meta">
        <span class="name">{item.name} <span class="badge">{item.scope}</span></span>
        <span class="sub ellipsis mono">{item.entry.command} {item.entry.args.join(" ")}</span>
      </span>
      <button class="push" on:click={() => uninstall(item)}>Remove</button>
    </div>
  {/each}
{/if}
<div class="section">Registry {#if $mcpLoading}<span class="spinner small"></span>{/if}</div>
{#each $mcpResults as server (server.id)}
  <div class="row">
    <span class="meta">
      <span class="name">{server.title} {#if server.plan}<span class="badge">{KIND[server.plan.kind]}</span>{/if}</span>
      <span class="desc">{server.description}</span>
      <span class="sub ellipsis">{server.id} · v{server.version}</span>
    </span>
    <span class="actions">
      {#if server.repository}
        <button class="icon" title="Open repository" on:click={() => openExternal(server.repository)}><Icon name="globe" size={12} /></button>
      {/if}
      <button class="push prominent" disabled={!server.plan} on:click={() => ($mcpInstallTarget = server)}>Install</button>
    </span>
  </div>
{:else}
  {#if !$mcpLoading}<div class="empty">No servers found.</div>{/if}
{/each}
{#if $mcpCursor}
  <button class="more" disabled={$mcpLoading} on:click={() => loadMore(query)}>Load more</button>
{/if}
{#if $mcpInstallTarget}
  <McpInstallSheet listing={$mcpInstallTarget} />
{/if}

<style>
  .reload { margin: 0 4px 8px; padding: 8px 10px; border-radius: 10px; display: flex; flex-direction: column; gap: 6px; font-size: var(--caption); background: rgba(var(--accent-rgb), 0.12); }
  .section { display: flex; align-items: center; gap: 6px; padding: 8px 8px 4px; font-size: var(--subheadline); font-weight: 700; text-transform: uppercase; letter-spacing: 0.4px; color: var(--secondary); }
  .row { display: flex; align-items: flex-start; gap: 8px; padding: 7px 8px; border-radius: 10px; }
  .row:hover { background: var(--hover); }
  .meta { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 2px; }
  .name { font-size: var(--body); font-weight: 600; }
  .badge { font-size: 9px; font-weight: 700; padding: 1px 5px; border-radius: 999px; color: var(--accent); background: rgba(var(--accent-rgb), 0.14); text-transform: uppercase; }
  .desc { font-size: var(--caption); color: var(--secondary); display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
  .sub { font-size: 10px; color: var(--tertiary); }
  .actions { display: flex; align-items: center; gap: 4px; }
  .icon { width: 22px; height: 22px; border-radius: 6px; display: flex; align-items: center; justify-content: center; color: var(--secondary); }
  .icon:hover { background: var(--hover); color: var(--label); }
  .push { font-size: var(--callout); padding: 2px 10px; }
  .more { margin: 8px; padding: 6px; border-radius: 8px; font-size: var(--callout); color: var(--accent); }
  .more:hover { background: var(--hover); }
  .empty { padding: 10px; font-size: var(--callout); color: var(--tertiary); }
  .spinner.small { width: 10px; height: 10px; }
</style>
