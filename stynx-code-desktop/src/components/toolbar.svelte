<script>
  import Icon from "./icon.svelte";
  import { info, status, isStreaming, showFiles, showSidebar, sessions } from "../lib/stores.js";
  import { openSession, startNewSession, refreshSessions } from "../lib/session-actions.js";
  import { isMac, popupMenu, startWindowDrag } from "../lib/native.js";

  $: subtitle = $info?.projectName || $info?.modelId || "";
  $: failed = $status === "Init failed";

  async function showHistory(event) {
    await refreshSessions();
    const items = $sessions.length
      ? $sessions.map((summary) => ({
          text: `${summary.title || "Untitled"}  —  ${summary.messageCount} messages`,
          action: () => openSession(summary.id),
        }))
      : [{ text: "No saved sessions", enabled: false }];
    popupMenu(items, event);
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<header class="toolbar" class:inset={isMac && !$showSidebar} on:mousedown={startWindowDrag}>
  {#if !$showSidebar}
    <button class="tool solo glass interactive" title="Show sidebar" on:click={() => ($showSidebar = true)}>
      <Icon name="sidebar.left" size={17} weight={1.6} />
    </button>
  {/if}
  <div class="titles">
    <span class="title">Stynx</span>
    {#if subtitle}<span class="subtitle ellipsis">{subtitle}</span>{/if}
  </div>
  <span class="spacer"></span>

  <div class="pill glass" class:busy={$isStreaming}>
    {#if $isStreaming}
      <span class="spinner small"></span>
    {:else}
      <span class="dot" class:failed></span>
    {/if}
    <span class="status">{$status}</span>
  </div>

  <div class="group glass">
    <button
      class="tool glass-item menu"
      title="Recent sessions"
      disabled={$isStreaming}
      on:click={showHistory}
    >
      <Icon name="clock.arrow.circlepath" size={16} weight={1.6} />
      <Icon name="chevron.down" size={8} weight={2.6} />
    </button>
    <button
      class="tool glass-item"
      title="New session"
      disabled={$isStreaming}
      on:click={startNewSession}
    >
      <Icon name="square.and.pencil" size={16} weight={1.6} />
    </button>
    <button class="tool glass-item" title="Toggle file browser" on:click={() => ($showFiles = !$showFiles)}>
      <Icon name="sidebar.right" size={17} weight={1.6} />
    </button>
  </div>
</header>

<style>
  /* Tahoe toolbar: no bar, just floating glass over a soft scroll-edge fade. */
  .toolbar {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: 20;
    height: var(--toolbar-h);
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 14px 0 18px;
  }

  .toolbar::before {
    content: "";
    position: absolute;
    inset: 0 0 -18px;
    z-index: -1;
    pointer-events: none;
    background: linear-gradient(
      to bottom,
      var(--text-bg) 0%,
      color-mix(in srgb, var(--text-bg) 75%, transparent) 55%,
      transparent 100%
    );
    backdrop-filter: blur(6px);
    -webkit-backdrop-filter: blur(6px);
    -webkit-mask-image: linear-gradient(to bottom, black 50%, transparent);
    mask-image: linear-gradient(to bottom, black 50%, transparent);
  }

  .toolbar.inset {
    padding-left: 84px;
  }

  .titles {
    display: flex;
    flex-direction: column;
    min-width: 0;
    line-height: 1.2;
  }

  .title {
    font-size: var(--headline);
    font-weight: 700;
  }

  .subtitle {
    font-size: var(--subheadline);
    color: var(--secondary);
  }

  .spacer {
    flex: 1;
  }

  .pill {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 34px;
    padding: 0 20px;
    border-radius: 999px;
    white-space: nowrap;
  }

  .status {
    font-size: var(--caption);
    font-weight: 500;
    color: var(--secondary);
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--green);
    box-shadow: 0 0 6px color-mix(in srgb, var(--green) 70%, transparent);
  }

  .dot.failed {
    background: var(--red);
    box-shadow: 0 0 6px color-mix(in srgb, var(--red) 70%, transparent);
  }

  .spinner.small {
    width: 10px;
    height: 10px;
  }

  .group {
    display: flex;
    align-items: center;
    gap: 2px;
    height: 34px;
    padding: 2px;
    border-radius: 999px;
  }

  .tool {
    height: 30px;
    min-width: 36px;
    padding: 0 9px;
    border-radius: 999px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 3px;
    color: var(--secondary);
  }

  .tool:hover:not(:disabled) {
    color: var(--label);
  }

  .tool.solo {
    height: 34px;
    min-width: 40px;
  }
</style>
