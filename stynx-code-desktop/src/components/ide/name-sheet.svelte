<script>
  import { onMount } from "svelte";
  import Sheet from "../sheet.svelte";
  import { namePrompt } from "../../lib/ide/ide-stores.js";

  export let prompt;

  let value = prompt.initial;
  let input;

  onMount(() => {
    input.focus();
    const dot = value.lastIndexOf(".");
    input.setSelectionRange(0, dot > 0 ? dot : value.length);
  });

  $: valid = value.trim().length > 0 && !value.includes("..") && !value.startsWith("/");

  function finish(result) {
    prompt.resolve(result);
    $namePrompt = null;
  }

  function submit() {
    if (valid) finish(value.trim());
  }
</script>

<Sheet width={400} onCancel={() => finish(null)}>
  <div class="body">
    <div class="headline">{prompt.title}</div>
    <input
      class="field"
      bind:this={input}
      bind:value
      on:keydown={(event) => event.key === "Enter" && submit()}
      placeholder="name or path/to/name"
      spellcheck="false"
    />
    <div class="actions">
      <button class="push" on:click={() => finish(null)}>Cancel</button>
      <span class="spacer"></span>
      <button class="push prominent" disabled={!valid} on:click={submit}>OK</button>
    </div>
  </div>
</Sheet>

<style>
  .body {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .headline {
    font-size: var(--headline);
    font-weight: 700;
  }

  .actions {
    display: flex;
    align-items: center;
  }

  .spacer {
    flex: 1;
  }
</style>
