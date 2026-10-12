<script>
  import { get } from "svelte/store";
  import { tick } from "svelte";
  import Icon from "./icon.svelte";
  import { isStreaming, composerDraft, composerFocus, pendingImages } from "../lib/stores.js";
  import { send } from "../lib/messaging.js";
  import { cancel } from "../lib/api.js";
  import { historyUp, historyDown } from "../lib/prompt-history.js";
  import { handlePaste, attachImages } from "../lib/composer-input.js";
  import { liquid } from "../lib/motion.js";
  import AttachmentsBar from "./attachments-bar.svelte";
  import MentionPopup from "./mention-popup.svelte";

  export let placeholder = "Ask stynx…";
  export let context = null;

  const LINE_HEIGHT = 18;
  const MAX_LINES = 14;

  let textarea;
  let focusSeen = $composerFocus;

  $: if ($composerFocus !== focusSeen) {
    focusSeen = $composerFocus;
    tick().then(() => textarea?.focus());
  }

  const sendable = (streaming, draft, images) =>
    !streaming && (draft.trim().length > 0 || images.length > 0);

  $: canSend = sendable($isStreaming, $composerDraft, $pendingImages);
  $: $composerDraft, autosize();

  async function autosize() {
    await tick();
    if (!textarea) return;
    const from = textarea.offsetHeight;
    textarea.style.transition = "none";
    textarea.style.height = "auto";
    const to = Math.min(textarea.scrollHeight, LINE_HEIGHT * MAX_LINES);
    textarea.style.height = `${from}px`;
    void textarea.offsetHeight;
    textarea.style.transition = "";
    textarea.style.height = `${to}px`;
  }

  function submit() {
    if (!sendable(get(isStreaming), get(composerDraft), get(pendingImages))) return;
    const text = $composerDraft;
    const images = get(pendingImages);
    $composerDraft = "";
    $pendingImages = [];
    send(text, images, context);
  }

  function onKeydown(event) {
    if (event.key === "Enter" && !event.shiftKey && !event.isComposing) {
      event.preventDefault();
      submit();
    } else if (event.key === "ArrowUp") {
      const previous = historyUp($composerDraft);
      if (previous !== null) {
        event.preventDefault();
        $composerDraft = previous;
      }
    } else if (event.key === "ArrowDown") {
      const next = historyDown();
      if (next !== null) {
        event.preventDefault();
        $composerDraft = next;
      }
    }
  }

</script>

<div class="composer glass panel">
  <MentionPopup />
  <AttachmentsBar />
  <slot name="above" />
  <textarea
    bind:this={textarea}
    bind:value={$composerDraft}
    on:keydown={onKeydown}
    on:paste={handlePaste}
    {placeholder}
    rows="1"
    spellcheck="false"
  ></textarea>
  <div class="actions">
    <button class="attach" title="Attach image" on:click={attachImages}>
      <Icon name="paperclip" size={16} weight={1.7} />
    </button>
    <span class="spacer"></span>
    {#if $isStreaming}
      <button class="round stop" title="Stop" on:click={() => cancel()} in:liquid={{ y: 0, scale: 0.5, duration: 560 }}>
        <Icon name="stop.circle.hierarchical" size={28} />
      </button>
    {:else}
      <button
        class="round"
        class:ready={canSend}
        title="Send"
        disabled={!canSend}
        on:click={submit}
        in:liquid={{ y: 0, scale: 0.5, duration: 560 }}
      >
        <Icon name="arrow.up.circle.hierarchical" size={28} />
      </button>
    {/if}
  </div>
</div>

<style>
  .composer {
    margin: 8px 16px 14px;
    padding: 14px;
    border-radius: 22px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    transition:
      box-shadow 0.45s var(--smooth),
      transform 0.6s var(--spring);
  }

  .composer:focus-within {
    transform: translateY(-1px);
    box-shadow:
      0 10px 32px rgba(0, 0, 0, 0.24),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  textarea {
    transition: height 0.32s var(--smooth);
    background: none;
    border: none;
    resize: none;
    font-size: var(--body);
    line-height: 18px;
    min-height: 24px;
    padding: 0;
    overflow-y: auto;
  }

  textarea::placeholder {
    color: var(--tertiary);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .spacer {
    flex: 1;
  }

  .attach {
    color: var(--secondary);
    display: flex;
  }

  .round {
    display: flex;
    color: var(--secondary);
  }

  .round:disabled {
    opacity: 1;
  }

  .round.ready {
    color: var(--accent);
    filter: drop-shadow(0 2px 6px rgba(var(--accent-rgb), 0.45));
  }

  .round.stop {
    color: var(--red);
  }
</style>
