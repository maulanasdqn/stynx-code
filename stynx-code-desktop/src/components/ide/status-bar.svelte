<script>
  import Icon from "../icon.svelte";
  import { activeSidePanel, diagnostics, ideMessage, lspStatus } from "../../lib/ide/ide-stores.js";

  $: counts = [...$diagnostics.values()].flat().reduce(
    (acc, d) => {
      if ((d.severity ?? 1) === 1) acc.errors += 1;
      else if (d.severity === 2) acc.warnings += 1;
      return acc;
    },
    { errors: 0, warnings: 0 },
  );
  $: servers = [...$lspStatus.entries()];
</script>

<footer class="bar">
  <button class="counts" title="Problems" on:click={() => ($activeSidePanel = "problems")}>
    <span class="error"><Icon name="exclamationmark.triangle.fill" size={10} /> {counts.errors}</span>
    <span class="warning"><Icon name="exclamationmark.triangle.fill" size={10} /> {counts.warnings}</span>
  </button>
  {#if $ideMessage}
    <span class="message ellipsis" title={$ideMessage}>{$ideMessage}</span>
  {/if}
  <span class="spacer"></span>
  {#each servers as [name, status] (name)}
    <span class="server {status.state}" title={status.message}>
      {#if status.state === "busy" || status.state === "starting"}<span class="spinner small"></span>{:else}<span class="dot"></span>{/if}
      <span class="ellipsis">{status.state === "busy" ? status.message : name}</span>
    </span>
  {/each}
</footer>

<style>
  .bar {
    height: 24px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 12px;
    font-size: var(--caption);
    color: var(--secondary);
    border-top: 1px solid var(--separator);
  }
  .counts { display: flex; gap: 8px; color: var(--secondary); }
  .counts span { display: flex; align-items: center; gap: 3px; }
  .error { color: var(--red); }
  .warning { color: var(--orange); }
  .message { min-width: 0; max-width: 40%; }
  .spacer { flex: 1; }
  .server { display: flex; align-items: center; gap: 5px; max-width: 260px; }
  .dot { width: 6px; height: 6px; border-radius: 50%; background: var(--green); flex-shrink: 0; }
  .server.missing .dot, .server.error .dot { background: var(--red); }
  .spinner.small { width: 9px; height: 9px; }
</style>
