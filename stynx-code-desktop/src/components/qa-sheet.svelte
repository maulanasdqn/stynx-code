<script>
  import Icon from "./icon.svelte";
  import QaRow from "./qa-row.svelte";
  import { respondAskUser } from "../lib/api.js";
  import { question as questionStore } from "../lib/stores.js";

  export let question;

  const OTHER = "__other__";

  let selections = {};
  let otherText = {};

  $: questions = question.qa;
  $: allAnswered = questions.every((_, index) => (selections[index] ?? []).length > 0);

  function toggle(index, label) {
    const qa = questions[index];
    const current = selections[index] ?? [];
    let next;
    if (qa.multiSelect) {
      next = current.includes(label) ? current.filter((l) => l !== label) : [...current, label];
    } else {
      next = [label];
    }
    selections = { ...selections, [index]: next };
  }

  function setOther(index, text) {
    otherText = { ...otherText, [index]: text };
    selections = { ...selections, [index]: [OTHER] };
  }

  async function respond(answer) {
    await respondAskUser(question.id, answer);
    $questionStore = null;
  }

  function send() {
    const lines = questions.map((qa, index) => {
      const picked = (selections[index] ?? []).map((label) =>
        label === OTHER ? (otherText[index] ?? "") : label,
      );
      return `${qa.header}: ${picked.join(", ")}`;
    });
    respond(lines.join("\n"));
  }
</script>

<div class="card">
  <div class="head">
    <Icon name="bubble.left.and.bubble.right" size={14} />
    <span class="title">Q&amp;A</span>
  </div>
  <div class="divider"></div>
  <div class="rows">
    {#each questions as qa, index}
      <QaRow
        question={qa}
        selected={selections[index] ?? []}
        other={otherText[index] ?? ""}
        onToggle={(label) => toggle(index, label)}
        onOther={(text) => setOther(index, text)}
      />
      {#if index < questions.length - 1}<div class="divider"></div>{/if}
    {/each}
  </div>
  <div class="divider"></div>
  <div class="actions">
    <button class="push large" on:click={() => respond("(skipped)")}>Skip</button>
    <button class="push large prominent" on:click={send} disabled={!allAnswered}>Send</button>
  </div>
</div>

<style>
  .card {
    border-radius: 12px;
    background: rgba(var(--fg-rgb), 0.05);
    border: 1px solid rgba(var(--accent-rgb), 0.35);
  }

  .head {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 14px;
  }

  .title {
    font-size: var(--headline);
    font-weight: 700;
  }

  .divider {
    height: 1px;
    background: var(--separator);
  }

  .rows {
    max-height: 380px;
    overflow-y: auto;
  }

  .actions {
    display: flex;
    gap: 10px;
    padding: 14px;
  }

  .actions button {
    flex: 1;
  }
</style>
