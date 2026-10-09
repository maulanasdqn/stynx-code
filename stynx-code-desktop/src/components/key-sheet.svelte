<script>
  import { onMount } from "svelte";
  import Icon from "./icon.svelte";
  import Sheet from "./sheet.svelte";

  export let entry;
  export let onSave;
  export let onCancel;

  let key = "";
  let input;

  onMount(() => input.focus());

  $: trimmed = key.trim();

  function save() {
    if (trimmed) onSave(trimmed);
  }
</script>

<Sheet width={460} {onCancel}>
  <div class="body">
    <div class="headline">
      <Icon name="key.fill" size={14} /> API key · {entry.provider}
    </div>
    <div class="hint">
      <span>Stored to ~/.stynx/keys.json and exported as</span>
      <span class="mono">{entry.keyEnv}</span>
    </div>
    <input
      class="field"
      type="password"
      placeholder="Paste API key…"
      bind:value={key}
      bind:this={input}
      on:keydown={(event) => event.key === "Enter" && save()}
      spellcheck="false"
    />
    <div class="actions">
      <button class="push" on:click={onCancel}>Cancel</button>
      <span class="spacer"></span>
      <button class="push prominent" on:click={save} disabled={!trimmed}>Save</button>
    </div>
  </div>
</Sheet>

<style>
  .body {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .headline {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: var(--headline);
    font-weight: 700;
  }

  .hint {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: var(--caption);
    color: var(--secondary);
  }

  .actions {
    display: flex;
    align-items: center;
  }

  .spacer {
    flex: 1;
  }
</style>
