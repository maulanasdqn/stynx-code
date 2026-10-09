<script>
  import { onMount } from "svelte";
  import Sidebar from "./components/sidebar.svelte";
  import Toolbar from "./components/toolbar.svelte";
  import Chat from "./components/chat.svelte";
  import FilesPanel from "./components/files-panel.svelte";
  import SplitHandle from "./components/split-handle.svelte";
  import { attachEngineEvents } from "./lib/engine-events.js";
  import { openWorkspace } from "./lib/session-actions.js";
  import { isMac, pickFolder } from "./lib/native.js";
  import { recentWorkspaces } from "./lib/workspaces.js";
  import { status, showFiles, showSidebar } from "./lib/stores.js";
  import { unfold, trackSpecular } from "./lib/motion.js";

  let bootError = "";
  let needsWorkspace = false;
  let sidebarWidth = 260;
  let filesFraction = 0.5;
  let detail;

  if (isMac) document.documentElement.classList.add("mac");

  onMount(async () => {
    await attachEngineEvents();
    const last = recentWorkspaces()[0];
    if (!(last && (await open(last, null)))) await chooseWorkspace();
  });

  async function chooseWorkspace() {
    const path = await pickFolder();
    needsWorkspace = !path;
    if (path) await open(path, null);
    else $status = "No workspace";
  }

  async function open(path, provider) {
    bootError = "";
    try {
      await openWorkspace(path, provider);
      needsWorkspace = false;
      return true;
    } catch (error) {
      bootError = String(error);
      $status = "Init failed";
      return false;
    }
  }

  function resizeSidebar(dx) {
    sidebarWidth = Math.min(320, Math.max(220, sidebarWidth + dx));
  }

  function resizeFiles(dx) {
    const width = detail.clientWidth;
    const files = Math.min(width - 400, Math.max(380, filesFraction * width - dx));
    filesFraction = files / width;
  }
</script>

<div class="window" use:trackSpecular>
  {#if $showSidebar}
    <div class="sidebar-col" style="width: {sidebarWidth + 12}px" transition:unfold>
      <div class="sidebar glass"><Sidebar onOpenWorkspace={open} /></div>
    </div>
    <SplitHandle onDrag={resizeSidebar} line={false} />
  {/if}
  <div class="detail" bind:this={detail}>
    <Toolbar />
    {#if bootError}
      <div class="boot-error">Could not start the engine: {bootError}</div>
    {/if}
    {#if needsWorkspace}
      <div class="welcome">
        <span class="welcome-text">No workspace open</span>
        <button class="push prominent" on:click={chooseWorkspace}>Open Workspace…</button>
      </div>
    {:else}
      <div class="workspace">
        <div class="chat-pane"><Chat /></div>
        {#if $showFiles}
          <SplitHandle onDrag={resizeFiles} line={false} />
          <div class="files-pane" style="width: {filesFraction * 100}%" transition:unfold>
            <div class="files-card glass panel"><FilesPanel /></div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .window {
    display: flex;
    height: 100%;
  }

  .sidebar-col {
    flex-shrink: 0;
    height: 100%;
    padding: 8px 4px 8px 8px;
    overflow: hidden;
  }

  .sidebar {
    height: 100%;
    border-radius: 20px;
    background: var(--sidebar-bg);
    box-shadow:
      0 6px 26px rgba(0, 0, 0, 0.22),
      0 0 0 1px rgba(0, 0, 0, 0.12),
      0 0 0 60px var(--text-bg);
  }

  :global(html.mac) .sidebar {
    background: linear-gradient(180deg, var(--glass-tint-2), transparent 40%);
    backdrop-filter: none;
    -webkit-backdrop-filter: none;
  }

  .detail {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    position: relative;
    background: var(--text-bg);
  }

  .workspace {
    flex: 1;
    display: flex;
    min-height: 0;
  }

  .chat-pane {
    flex: 1;
    min-width: 400px;
    display: flex;
  }

  .files-pane {
    min-width: 380px;
    display: flex;
    padding: calc(var(--toolbar-h) + 2px) 8px 8px 0;
  }

  .files-card {
    flex: 1;
    min-width: 0;
    display: flex;
    border-radius: 20px;
    overflow: hidden;
  }

  .welcome {
    flex: 1;
    padding-top: var(--toolbar-h);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
  }

  .welcome-text {
    font-size: var(--title3);
    color: var(--secondary);
  }

  .boot-error {
    margin: calc(var(--toolbar-h) + 8px) 16px -40px;
    position: relative;
    z-index: 5;
    padding: 8px 12px;
    border-radius: 8px;
    background: rgba(255, 69, 58, 0.12);
    border: 1px solid rgba(255, 69, 58, 0.4);
    color: var(--red);
    font-size: var(--callout);
  }
</style>
