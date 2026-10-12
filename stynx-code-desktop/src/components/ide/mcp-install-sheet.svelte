<script>
  import Sheet from "../sheet.svelte";
  import { buildEntry, install, mcpInstallTarget } from "../../lib/ide/mcp.js";

  export let listing;

  const plan = listing.plan;
  let name = listing.name;
  let scope = "workspace";
  let values = Object.fromEntries(plan.inputs.map((input) => [input.key, input.default ?? ""]));
  let error = "";
  let busy = false;

  $: missing = plan.inputs.filter((input) => input.required && !String(values[input.key] ?? "").trim());
  $: preview = (() => {
    const masked = Object.fromEntries(plan.inputs.map((i) => [i.key, i.secret && values[i.key] ? "••••" : values[i.key] || `<${i.key}>`]));
    const entry = buildEntry(plan, masked);
    return [...Object.keys(entry.env).map((k) => `${k}=${entry.env[k]}`), entry.command, ...entry.args].join(" ");
  })();

  const close = () => ($mcpInstallTarget = null);

  async function submit() {
    if (missing.length || !name.trim()) return;
    busy = true;
    error = "";
    try {
      await install(listing, scope, name.trim(), values);
      close();
    } catch (cause) {
      error = String(cause);
    } finally {
      busy = false;
    }
  }
</script>

<Sheet width={480} onCancel={close}>
  <div class="body">
    <div class="headline">Install {listing.title}</div>
    <div class="desc">{listing.description}</div>
    <label class="field-row">
      <span>Name</span>
      <input class="field mono" bind:value={name} spellcheck="false" />
    </label>
    <div class="field-row">
      <span>Scope</span>
      <div class="scopes">
        <button class:active={scope === "workspace"} on:click={() => (scope = "workspace")}>This workspace</button>
        <button class:active={scope === "user"} on:click={() => (scope = "user")}>All workspaces</button>
      </div>
    </div>
    {#each plan.inputs as input (input.key)}
      <label class="field-row">
        <span>{input.label}{input.required ? " *" : ""}</span>
        {#if input.secret}
          <input class="field" type="password" bind:value={values[input.key]} placeholder={input.description || input.key} spellcheck="false" />
        {:else}
          <input class="field" bind:value={values[input.key]} placeholder={input.description || input.key} spellcheck="false" />
        {/if}
      </label>
    {/each}
    <pre class="preview">{preview}</pre>
    <div class="note">Saved to {scope === "workspace" ? ".mcp.json in this workspace" : "~/.claude/mcp.json"}.</div>
    {#if error}<div class="error">{error}</div>{/if}
    <div class="actions">
      <button class="push" on:click={close}>Cancel</button>
      <span class="spacer"></span>
      <button class="push prominent" disabled={busy || missing.length > 0 || !name.trim()} on:click={submit}>
        {busy ? "Installing…" : "Install"}
      </button>
    </div>
  </div>
</Sheet>

<style>
  .body { display: flex; flex-direction: column; gap: 10px; }
  .headline { font-size: var(--headline); font-weight: 700; }
  .desc { font-size: var(--callout); color: var(--secondary); }
  .field-row { display: flex; flex-direction: column; gap: 4px; font-size: var(--callout); }
  .field-row > span { color: var(--secondary); }
  .scopes { display: flex; gap: 4px; }
  .scopes button { flex: 1; padding: 4px 8px; border-radius: 7px; font-size: var(--callout); color: var(--secondary); background: rgba(var(--fg-rgb), 0.05); }
  .scopes button.active { color: white; background: var(--accent); }
  .preview { padding: 8px 10px; border-radius: 8px; font-family: var(--mono); font-size: 10.5px; white-space: pre-wrap; word-break: break-all; color: var(--secondary); background: rgba(var(--fg-rgb), 0.05); }
  .note { font-size: var(--caption); color: var(--tertiary); }
  .error { font-size: var(--callout); color: var(--red); }
  .actions { display: flex; align-items: center; }
  .spacer { flex: 1; }
</style>
