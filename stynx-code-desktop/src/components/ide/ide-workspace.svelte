<script>
  import { onDestroy, onMount } from "svelte";
  import ActivityRail from "./activity-rail.svelte";
  import ExplorerPanel from "./explorer-panel.svelte";
  import GitPanel from "./git-panel.svelte";
  import ProblemsList from "./problems-list.svelte";
  import ExtensionsPanel from "./extensions-panel.svelte";
  import InternsPanel from "./interns-panel.svelte";
  import SearchPanel from "./search-panel.svelte";
  import StatusBar from "./status-bar.svelte";
  import EditorArea from "./editor-area.svelte";
  import TerminalPanel from "./terminal-panel.svelte";
  import QuickOpen from "./quick-open.svelte";
  import NameSheet from "./name-sheet.svelte";
  import Chat from "../chat.svelte";
  import SplitHandle from "../split-handle.svelte";
  import { info } from "../../lib/stores.js";
  import { activeSidePanel, ideRoot, namePrompt, pendingOpen, quickOpenVisible, relativeToRoot, terminals, terminalVisible, tree } from "../../lib/ide/ide-stores.js";
  import { openFile } from "../../lib/ide/documents.js";
  import { initMonaco } from "../../lib/ide/monaco-setup.js";
  import { attachWorkspace, startFsEvents } from "../../lib/ide/workspace-session.js";
  import { installKeybindings } from "../../lib/ide/keybindings.js";
  import { installLsp } from "../../lib/ide/lsp-providers.js";

  export let visible = true;

  let sideWidth = 260;
  let agentWidth = 380;
  let terminalHeight = 260;
  let showAgent = true;
  let attachedFor = null;
  let uninstall = () => {};

  initMonaco();

  onMount(() => {
    uninstall = installKeybindings();
    startFsEvents();
    installLsp(openFile);
  });

  onDestroy(() => uninstall());

  $: if ($info?.workspacePath && $info.workspacePath !== attachedFor) {
    attachedFor = $info.workspacePath;
    attachWorkspace();
  }

  $: if ($pendingOpen && $ideRoot && $tree.has("")) {
    const relative = relativeToRoot($pendingOpen);
    $pendingOpen = null;
    if (relative) openFile(relative);
  }

  const clamp = (value, min, max) => Math.min(max, Math.max(min, value));
</script>

<div class="ide" class:hidden={!visible}>
  <ActivityRail bind:showAgent />
  {#if $activeSidePanel}
    <div class="side glass panel" style="width: {sideWidth}px">
      {#if $activeSidePanel === "git"}<GitPanel />{:else if $activeSidePanel === "problems"}<ProblemsList />{:else if $activeSidePanel === "extensions"}<ExtensionsPanel />{:else if $activeSidePanel === "interns"}<InternsPanel />{:else if $activeSidePanel === "search"}<SearchPanel />{:else}<ExplorerPanel />{/if}
    </div>
    <SplitHandle onDrag={(dx) => (sideWidth = clamp(sideWidth + dx, 180, 480))} line={false} />
  {/if}
  <div class="center">
    <div class="editor glass panel">
      <EditorArea />
      <StatusBar />
    </div>
    {#if $terminals.length > 0}
      {#if $terminalVisible}
        <SplitHandle vertical onDrag={(dy) => (terminalHeight = clamp(terminalHeight - dy, 120, 640))} line={false} />
      {/if}
      <div class="terminal glass panel" class:collapsed={!$terminalVisible} style="height: {terminalHeight}px">
        <TerminalPanel />
      </div>
    {/if}
  </div>
  {#if showAgent}
    <SplitHandle onDrag={(dx) => (agentWidth = clamp(agentWidth - dx, 320, 640))} line={false} />
    <div class="agent glass panel" style="width: {agentWidth}px">
      {#if visible}<Chat />{/if}
    </div>
  {/if}
</div>

{#if visible && $quickOpenVisible}
  <QuickOpen />
{/if}
{#if $namePrompt}
  <NameSheet prompt={$namePrompt} />
{/if}

<style>
  .ide {
    flex: 1;
    min-height: 0;
    display: flex;
    gap: 0;
    padding: calc(var(--toolbar-h) + 2px) 8px 8px 8px;
  }

  .ide.hidden {
    display: none;
  }

  .side,
  .editor,
  .agent {
    border-radius: 16px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .side {
    flex-shrink: 0;
  }

  .center {
    flex: 1;
    min-width: 320px;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .editor {
    flex: 1;
  }

  .terminal {
    flex-shrink: 0;
    border-radius: 16px;
    overflow: hidden;
    display: flex;
    min-height: 0;
  }

  .terminal.collapsed {
    display: none;
  }

  .agent {
    flex-shrink: 0;
    position: relative;
    --toolbar-h: 0px;
  }
</style>
