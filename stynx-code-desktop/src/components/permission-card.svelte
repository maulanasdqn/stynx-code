<script>
  import Icon from "./icon.svelte";
  import Markdown from "./markdown.svelte";
  import { respondPermission } from "../lib/api.js";
  import { permissionPrompt } from "../lib/stores.js";

  export let prompt;

  async function respond(choice) {
    await respondPermission(prompt.id, choice);
    $permissionPrompt = null;
  }

  // .keyboardShortcut(.defaultAction) on "Allow Once" — unless the user is typing.
  function keydown(event) {
    if (event.key !== "Enter" || event.target.closest("textarea, input")) return;
    event.preventDefault();
    respond("allow_once");
  }
</script>

<svelte:window on:keydown={keydown} />

<div class="card">
  <div class="headline"><Icon name="lock.shield" size={13} /> Permission required</div>
  <div class="tool mono">{prompt.toolName}</div>
  {#if prompt.description}
    <div class="description">
      <Markdown raw={prompt.description} />
    </div>
  {/if}
  <div class="actions">
    <button class="push" on:click={() => respond("deny")}>Deny</button>
    <span class="spacer"></span>
    <button class="push" on:click={() => respond("allow_always")}>Allow Always</button>
    <button class="push prominent" on:click={() => respond("allow_once")}>Allow Once</button>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 14px;
    border-radius: 12px;
    background: rgba(var(--fg-rgb), 0.05);
    border: 1px solid color-mix(in srgb, var(--orange) 40%, transparent);
  }

  .headline {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: var(--subheadline);
    font-weight: 600;
  }

  .tool {
    font-size: var(--callout);
    color: var(--secondary);
  }

  .description {
    max-height: 180px;
    overflow-y: auto;
    padding: 10px;
    border-radius: 8px;
    background: var(--text-bg);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .spacer {
    flex: 1;
  }
</style>
