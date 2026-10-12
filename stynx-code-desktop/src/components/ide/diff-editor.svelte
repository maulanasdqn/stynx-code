<script>
  import { onDestroy, onMount } from "svelte";
  import { monaco } from "../../lib/ide/monaco-setup.js";
  import { langFromPath } from "../../lib/ide/monaco-languages.js";

  export let diff;

  let host;
  let editor;
  let models = [];

  onMount(() => {
    editor = monaco.editor.createDiffEditor(host, {
      automaticLayout: true,
      readOnly: true,
      originalEditable: false,
      renderSideBySide: true,
      useInlineViewWhenSpaceIsLimited: true,
      fontFamily: 'ui-monospace, "SF Mono", Menlo, monospace',
      fontSize: 13,
      lineHeight: 20,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      renderOverviewRuler: true,
      hideUnchangedRegions: { enabled: true, contextLineCount: 3, minimumLineCount: 5 },
      padding: { top: 8 },
    });
    show(diff);
  });

  onDestroy(() => {
    editor?.dispose();
    for (const model of models) model.dispose();
  });

  function show(next) {
    if (!editor || !next) return;
    const language = langFromPath(next.path);
    const original = monaco.editor.createModel(next.original, language);
    const modified = monaco.editor.createModel(next.modified, language);
    editor.setModel({ original, modified });
    for (const model of models) model.dispose();
    models = [original, modified];
  }

  $: show(diff);
</script>

{#if diff.binary || diff.tooLarge}
  <div class="note">{diff.binary ? "Binary file — no text diff." : "File is too large to diff."}</div>
{:else}
  <div class="host" bind:this={host}></div>
{/if}

<style>
  .host {
    flex: 1;
    min-height: 0;
  }

  .note {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--callout);
    color: var(--secondary);
  }
</style>
