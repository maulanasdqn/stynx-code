<script>
  import Icon from "./icon.svelte";
  import Markdown from "./markdown.svelte";
  import { respondAskUser } from "../lib/api.js";
  import { question as questionStore } from "../lib/stores.js";

  export let question;

  let draft = "";

  $: trimmed = draft.trim();

  async function respond(answer) {
    await respondAskUser(question.id, answer);
    $questionStore = null;
  }

  function keydown(event) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      if (trimmed) respond(draft);
    } else if (event.key === "Escape") {
      respond(null);
    }
  }
</script>

<div class="card">
  <div class="headline"><Icon name="questionmark.bubble" size={14} /> Stynx asks</div>
  <div class="question">
    <Markdown raw={question.question} />
  </div>
  <textarea
    class="field"
    rows="1"
    placeholder="Your answer…"
    bind:value={draft}
    on:keydown={keydown}
  ></textarea>
  <div class="actions">
    <button class="push" on:click={() => respond(null)}>Cancel</button>
    <span class="spacer"></span>
    <button class="push prominent" on:click={() => respond(draft)} disabled={!trimmed}>Send</button>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 16px;
    border-radius: 12px;
    background: rgba(var(--fg-rgb), 0.05);
    border: 1px solid rgba(var(--accent-rgb), 0.35);
  }

  .headline {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: var(--headline);
    font-weight: 700;
  }

  .question {
    max-height: 320px;
    overflow-y: auto;
  }

  textarea {
    resize: none;
    field-sizing: content;
    max-height: 120px;
    background: var(--text-bg);
  }

  .actions {
    display: flex;
    align-items: center;
  }

  .spacer {
    flex: 1;
  }
</style>
