<script>
  import { onMount, tick } from "svelte";
  import FeedItem from "./feed-item.svelte";
  import PermissionCard from "./permission-card.svelte";
  import QuestionSheet from "./question-sheet.svelte";
  import QaSheet from "./qa-sheet.svelte";
  import TypingDots from "./typing-dots.svelte";
  import QueuePanel from "./queue-panel.svelte";
  import Composer from "./composer.svelte";
  import { feed, permissionPrompt, question, isStreaming } from "../lib/stores.js";
  import { liquid } from "../lib/motion.js";

  export let placeholder = undefined;
  export let context = null;

  let scroller;
  let stack;
  let composerHeight = 120;
  let pinned = true;
  let seen = 0;

  $: follow($feed, $permissionPrompt, $question, $isStreaming, composerHeight);

  $: last = $feed[$feed.length - 1];
  $: showTyping =
    $isStreaming && !(last && (last.role === "assistant" || last.role === "thinking") && last.text);

  async function follow(items) {
    const added = items.length !== seen;
    if (added && (items.length < seen || items.at(-1)?.role === "user")) pinned = true;
    seen = items.length;
    await tick();
    stick();
  }

  function stick() {
    if (pinned && scroller) scroller.scrollTop = scroller.scrollHeight;
  }

  function onScroll() {
    pinned = scroller.scrollHeight - scroller.scrollTop - scroller.clientHeight < 60;
  }

  onMount(() => {
    const observer = new ResizeObserver(stick);
    observer.observe(stack);
    observer.observe(scroller);
    return () => observer.disconnect();
  });
</script>

<div class="chat" style="--composer-h: {composerHeight}px">
  <div class="transcript" bind:this={scroller} on:scroll={onScroll}>
    <div class="stack" bind:this={stack}>
    {#if $feed.length === 0 && !$permissionPrompt && !$question && !$isStreaming}
      <slot name="empty" />
    {/if}
    {#each $feed as item (item.id)}
      <div class="entry"><FeedItem {item} /></div>
    {/each}
    {#if $permissionPrompt}
      <div class="entry" out:liquid={{ duration: 260 }}>
        <PermissionCard prompt={$permissionPrompt} />
      </div>
    {/if}
    {#if $question}
      <div class="entry" out:liquid={{ duration: 260 }}>
        {#if $question.qa?.length}
          <QaSheet question={$question} />
        {:else}
          <QuestionSheet question={$question} />
        {/if}
      </div>
    {/if}
    {#if showTyping}
      <div class="entry typing" out:liquid={{ duration: 200, y: 0 }}><TypingDots /></div>
    {/if}
    </div>
  </div>

  <QueuePanel />
  <div class="composer-dock" bind:clientHeight={composerHeight}><Composer {placeholder} {context}>
    <svelte:fragment slot="above"><slot name="composer-above" /></svelte:fragment>
  </Composer></div>
</div>

<style>
  .chat {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    position: relative;
    background: var(--text-bg);
  }

  .transcript {
    flex: 1;
    overflow-y: auto;
    padding: calc(var(--toolbar-h) + 14px) 20px calc(var(--composer-h) + 24px);
  }

  .stack {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 14px;
  }

  .entry {
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    animation: surface 0.6s var(--spring) backwards;
  }

  @keyframes surface {
    from {
      opacity: 0;
      transform: translateY(10px) scale(0.97);
      filter: blur(6px);
    }
    40% {
      opacity: 1;
      filter: blur(0);
    }
  }

  .entry.typing {
    align-items: flex-start;
  }

  .composer-dock {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 10;
  }
</style>
