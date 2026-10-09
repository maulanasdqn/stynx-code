<script>
  import Icon from "./icon.svelte";
  import { liquid } from "../lib/motion.js";

  export let question;
  export let selected;
  export let other;
  export let onToggle;
  export let onOther;

  const OTHER = "__other__";

  let open = false;
  let showOtherField = false;
  let draft = other;
  let place = "";
  let rowButton;

  function toggleOpen(event) {
    const rect = event.currentTarget.getBoundingClientRect();
    const left = Math.max(8, Math.min(rect.right - 374, window.innerWidth - 368));
    const top = Math.max(8, Math.min(rect.top, window.innerHeight - 360));
    place = `left: ${left}px; top: ${top}px`;
    open = !open;
  }

  $: isChosen = selected.length > 0;
  $: display = selected.includes(OTHER)
    ? other || "Other…"
    : selected.length
      ? selected.join(", ")
      : "Choose…";

  function pick(label) {
    showOtherField = label === OTHER;
    onToggle(label);
    if (!question.multiSelect && label !== OTHER) open = false;
  }

  function floating(node) {
    document.body.appendChild(node);
    const handler = (event) => {
      if (!node.contains(event.target) && !rowButton?.contains(event.target)) open = false;
    };
    setTimeout(() => document.addEventListener("mousedown", handler));
    return {
      destroy: () => {
        document.removeEventListener("mousedown", handler);
        node.remove();
      },
    };
  }
</script>

<div class="anchor">
  <button class="row" bind:this={rowButton} on:click={toggleOpen}>
    <span class="text">
      <span class="header">{question.header}</span>
      <span class="value" class:chosen={isChosen}>{display}</span>
    </span>
    <span class="chevron"><Icon name="chevron.right" size={10} weight={2.4} /></span>
  </button>

  {#if open}
    <div class="popover" style={place} use:floating transition:liquid={{ y: -6, scale: 0.94, duration: 460 }}>
      <div class="question">{question.question}</div>
      {#each question.options as option}
        <button
          class="option"
          class:selected={selected.includes(option.label)}
          on:click={() => pick(option.label)}
        >
          <span class="option-title">{option.label}</span>
          {#if option.description}<span class="option-desc">{option.description}</span>{/if}
        </button>
      {/each}
      <button class="option" class:selected={selected.includes(OTHER)} on:click={() => pick(OTHER)}>
        <span class="option-title">Other</span>
      </button>
      {#if showOtherField || selected.includes(OTHER)}
        <input
          class="field"
          placeholder="Type your answer…"
          bind:value={draft}
          on:input={() => onOther(draft)}
        />
      {/if}
    </div>
  {/if}
</div>

<style>
  .anchor {
    position: relative;
  }

  .row {
    width: 100%;
    display: flex;
    align-items: center;
    padding: 12px 14px;
    text-align: left;
  }

  .text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .header {
    font-size: var(--caption);
    color: var(--secondary);
  }

  .value {
    font-size: var(--body);
    font-weight: 600;
    color: var(--secondary);
  }

  .value.chosen {
    color: var(--label);
  }

  .chevron {
    color: var(--secondary);
  }

  .popover {
    position: fixed;
    z-index: 50;
    max-height: calc(100vh - 16px);
    overflow-y: auto;
    width: 360px;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    border-radius: 12px;
    background: var(--material-regular);
    backdrop-filter: blur(40px) saturate(1.8);
    -webkit-backdrop-filter: blur(40px) saturate(1.8);
    border: 1px solid var(--glass-edge);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
  }

  .question {
    font-size: var(--headline);
    font-weight: 700;
  }

  .option {
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding: 10px;
    border-radius: 10px;
    text-align: left;
    background: rgba(var(--fg-rgb), 0.04);
    border: 1.5px solid transparent;
  }

  .option.selected {
    background: rgba(var(--accent-rgb), 0.22);
    border-color: var(--accent);
  }

  .option-title {
    font-weight: 600;
  }

  .option-desc {
    font-size: var(--caption);
    color: var(--secondary);
  }
</style>
