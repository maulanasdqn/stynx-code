<script>
  import Icon from "../icon.svelte";
  import EditorTabs from "./editor-tabs.svelte";
  import CodeEditor from "./code-editor.svelte";
  import ConflictBanner from "./conflict-banner.svelte";
  import { activePath, quickOpenVisible, tabs } from "../../lib/ide/ide-stores.js";

  $: tab = $tabs.find((entry) => entry.path === $activePath) ?? null;
  $: textPath = tab?.kind === "text" ? tab.path : null;
</script>

<div class="area">
  {#if $tabs.length > 0}
    <EditorTabs />
  {/if}
  {#if tab?.conflict}
    <ConflictBanner path={tab.path} />
  {/if}
  <div class="body">
    <CodeEditor path={textPath} />
    {#if !tab}
      <div class="placeholder">
        <Icon name="chevron.left.forwardslash.chevron.right" size={34} weight={1.3} />
        <span class="headline">No file open</span>
        <button class="push" on:click={() => ($quickOpenVisible = true)}>Go to File… ⌘P</button>
      </div>
    {:else if tab.kind !== "text"}
      <div class="placeholder">
        <Icon name="doc" size={34} weight={1.3} />
        <span class="headline">{tab.name}</span>
        <span class="hint">{tab.kind === "binary" ? "Binary file — not shown in the editor." : "File is larger than 5 MB."}</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .area {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .body {
    flex: 1;
    min-height: 0;
    display: flex;
    position: relative;
  }

  .placeholder {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    color: var(--tertiary);
    background: var(--text-bg);
  }

  .headline {
    font-size: var(--title3);
    color: var(--secondary);
  }

  .hint {
    font-size: var(--callout);
    color: var(--secondary);
  }
</style>
