<script>
  import Sheet from "../sheet.svelte";
  import { PROVIDERS, internEditor, saveIntern } from "../../lib/ide/interns.js";

  export let editor;

  let entry = { ...editor.entry };
  let apiKey = "";
  let error = "";
  let busy = false;

  $: preset = PROVIDERS.find((p) => p.id === entry.provider) ?? PROVIDERS.at(-1);
  $: keyEnv = entry.apiKeyEnv || preset.keyEnv;
  $: valid = entry.name.trim() && entry.model.trim() && (entry.provider !== "custom" || (entry.baseUrl?.trim() && entry.apiKeyEnv?.trim()));

  const close = () => ($internEditor = null);

  async function submit() {
    if (!valid) return;
    busy = true;
    error = "";
    try {
      await saveIntern({ ...entry, name: entry.name.trim() }, editor.previous, apiKey);
      close();
    } catch (cause) {
      error = String(cause);
    } finally {
      busy = false;
    }
  }
</script>

<Sheet width={460} onCancel={close}>
  <div class="body">
    <div class="headline">{editor.previous ? `Edit ${editor.previous}` : "Add Intern"}</div>
    <label class="row"><span>Name</span><input class="field mono" bind:value={entry.name} placeholder="netra" spellcheck="false" /></label>
    <label class="row">
      <span>Provider</span>
      <select bind:value={entry.provider}>
        {#each PROVIDERS as provider}<option value={provider.id}>{provider.label}</option>{/each}
      </select>
    </label>
    <label class="row"><span>Model</span><input class="field mono" bind:value={entry.model} placeholder="deepseek/deepseek-v4-flash" spellcheck="false" /></label>
    {#if entry.provider === "custom"}
      <label class="row"><span>Base URL</span><input class="field mono" bind:value={entry.baseUrl} placeholder="https://api.example.com/v1" spellcheck="false" /></label>
    {/if}
    <label class="row">
      <span>Key variable</span>
      <input class="field mono" bind:value={entry.apiKeyEnv} placeholder={preset.keyEnv || "MY_PROVIDER_API_KEY"} spellcheck="false" />
    </label>
    <label class="row">
      <span>API key {keyEnv ? `(${keyEnv})` : ""}</span>
      <input class="field" type="password" bind:value={apiKey} placeholder="Leave empty to keep the current key" spellcheck="false" />
    </label>
    <label class="row"><span>Description</span><input class="field" bind:value={entry.description} placeholder="What the agent should use it for" /></label>
    {#if error}<div class="error">{error}</div>{/if}
    <div class="actions">
      <button class="push" on:click={close}>Cancel</button>
      <span class="spacer"></span>
      <button class="push prominent" disabled={!valid || busy} on:click={submit}>{busy ? "Saving…" : "Save"}</button>
    </div>
  </div>
</Sheet>

<style>
  .body { display: flex; flex-direction: column; gap: 10px; }
  .headline { font-size: var(--headline); font-weight: 700; }
  .row { display: flex; flex-direction: column; gap: 4px; font-size: var(--callout); }
  .row > span { color: var(--secondary); }
  select { font: inherit; font-size: var(--body); }
  .error { font-size: var(--callout); color: var(--red); }
  .actions { display: flex; align-items: center; }
  .spacer { flex: 1; }
</style>
