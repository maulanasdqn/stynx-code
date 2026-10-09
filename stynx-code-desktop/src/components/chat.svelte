<script>
  import { tick } from "svelte";
  import FeedItem from "./feed-item.svelte";
  import PermissionCard from "./permission-card.svelte";
  import QuestionSheet from "./question-sheet.svelte";
  import QaSheet from "./qa-sheet.svelte";
  import TypingDots from "./typing-dots.svelte";
  import QueuePanel from "./queue-panel.svelte";
  import Composer from "./composer.svelte";
  import { feed, permissionPrompt, question, isStreaming } from "../lib/stores.js";
  import { liquid } from "../lib/motion.js";

  let scroller;
  let composerHeight = 120;

  $: $feed, $permissionPrompt, $question, $isStreaming, scrollToBottom();

  // Hide the dots once the assistant (or its thinking) has started producing text.
  $: last = $feed[$feed.length - 1];
  $: showTyping =
    $isStreaming && !(last && (last.role === "assistant" || last.role === "thinking") && last.text);

  async function scrollToBottom() {
    await tick();
    if (scroller) scroller.scrollTo({ top: scroller.scrollHeight, behavior: "smooth" });
  }
</script>

<div class="chat" style="--composer-h: {composerHeight}px">
  <div class="transcript" bind:this={scroller}>
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

  <QueuePanel />
  <div class="composer-dock" bind:clientHeight={composerHeight}><Composer /></div>
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

  /* Entries surface out of the glass: rise, unblur, settle with a soft overshoot. */
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

  /* The composer floats over the transcript so messages slide under the glass. */
  .composer-dock {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 10;
  }
</style>
