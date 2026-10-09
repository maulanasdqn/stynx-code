<script>
  import Icon from "./icon.svelte";
  import Markdown from "./markdown.svelte";
  import ActionCard from "./action-card.svelte";
  import CrossWorkspaceCard from "./cross-workspace-card.svelte";
  import ThinkingView from "./thinking-view.svelte";
  import { dataUrl } from "../lib/images.js";

  export let item;
</script>

{#if item.role === "user"}
  <div class="user-row">
    <div class="user-stack">
      {#if item.images?.length}
        <div class="user-images">
          {#each item.images as image}
            <img src={dataUrl(image)} alt="attachment" />
          {/each}
        </div>
      {/if}
      {#if item.text}
        <div class="bubble selectable">{item.text}</div>
      {/if}
      {#if item.referenceCount > 0}
        <div class="refs">
          <Icon name="doc.badge.plus" size={11} />
          {item.referenceCount} reference{item.referenceCount === 1 ? "" : "s"}
        </div>
      {/if}
    </div>
  </div>
{:else if item.role === "assistant"}
  <div class="assistant">
    <div class="role">Stynx</div>
    <Markdown raw={item.text} />
  </div>
{:else if item.role === "thinking"}
  <ThinkingView text={item.text} />
{:else if item.role === "tool"}
  {#if item.tool.name === "message_workspace"}
    <CrossWorkspaceCard tool={item.tool} incoming={false} />
  {:else if item.tool.name === "incoming_workspace"}
    <CrossWorkspaceCard tool={item.tool} incoming={true} />
  {:else}
    <ActionCard tool={item.tool} />
  {/if}
{:else if item.role === "compact"}
  <div class="compact">
    <span class="rule"></span>
    <span class="compact-label">
      <Icon name="arrow.counterclockwise" size={10} />
      Context compacted · {item.originalTurns} turns summarised
    </span>
    <span class="rule"></span>
  </div>
{/if}

<style>
  .user-row {
    display: flex;
    justify-content: flex-end;
    padding-left: 60px;
  }

  .user-stack {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 6px;
    min-width: 0;
  }

  .user-images {
    display: flex;
    gap: 8px;
  }

  .user-images img {
    width: 120px;
    height: 120px;
    object-fit: cover;
    border-radius: 12px;
  }

  .bubble {
    background: rgba(var(--accent-rgb), 0.9);
    color: white;
    padding: 9px 14px;
    border-radius: 16px;
    font-size: var(--body);
    line-height: 1.4;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .refs {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: var(--caption);
    color: var(--secondary);
  }

  .assistant {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .role {
    font-size: var(--caption);
    font-weight: 600;
    letter-spacing: 0.6px;
    text-transform: uppercase;
    color: var(--secondary);
  }

  .compact {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
  }

  .rule {
    flex: 1;
    height: 1px;
    background: var(--quaternary);
  }

  .compact-label {
    display: flex;
    align-items: center;
    gap: 4px;
    white-space: nowrap;
    font-size: var(--caption);
    color: var(--tertiary);
  }
</style>
