<script>
  import Chat from "./chat.svelte";
  import FilesPanel from "./files-panel.svelte";
  import SplitHandle from "./split-handle.svelte";
  import { showFiles } from "../lib/stores.js";
  import { unfold } from "../lib/motion.js";

  let workspace;
  let filesFraction = 0.5;

  function resizeFiles(dx) {
    const width = workspace.clientWidth;
    const files = Math.min(width - 400, Math.max(380, filesFraction * width - dx));
    filesFraction = files / width;
  }
</script>

<div class="workspace" bind:this={workspace}>
  <div class="chat-pane"><Chat /></div>
  {#if $showFiles}
    <SplitHandle onDrag={resizeFiles} line={false} />
    <div class="files-pane" style="width: {filesFraction * 100}%" transition:unfold>
      <div class="files-card glass panel"><FilesPanel /></div>
    </div>
  {/if}
</div>

<style>
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
</style>
