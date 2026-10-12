<script>
  import { onDestroy, onMount } from "svelte";
  import { monaco } from "../../lib/ide/monaco-setup.js";
  import { modelFor } from "../../lib/ide/documents.js";
  import { revealTarget } from "../../lib/ide/ide-stores.js";

  export let path;

  let host;
  let editor;
  let current = null;
  const viewStates = new Map();

  onMount(() => {
    editor = monaco.editor.create(host, {
      automaticLayout: true,
      fontFamily: 'ui-monospace, "SF Mono", Menlo, monospace',
      fontSize: 13,
      lineHeight: 20,
      minimap: { enabled: true, renderCharacters: false, scale: 1 },
      scrollBeyondLastLine: false,
      smoothScrolling: true,
      cursorSmoothCaretAnimation: "on",
      cursorBlinking: "smooth",
      renderLineHighlight: "line",
      bracketPairColorization: { enabled: true },
      guides: { bracketPairs: "active", indentation: true },
      padding: { top: 10, bottom: 10 },
      stickyScroll: { enabled: true },
      fixedOverflowWidgets: true,
      model: null,
    });
    show(path);
  });

  onDestroy(() => editor?.dispose());

  function show(next) {
    if (!editor || next === current) return;
    if (current) viewStates.set(current, editor.saveViewState());
    const model = next ? modelFor(next) : null;
    editor.setModel(model);
    current = next;
    if (model) {
      const state = viewStates.get(next);
      if (state) editor.restoreViewState(state);
      editor.focus();
    }
  }

  function reveal(target) {
    if (!editor || !target || target.path !== current) return;
    const position = { lineNumber: target.line, column: target.column };
    editor.setPosition(position);
    editor.revealPositionInCenter(position);
    editor.focus();
    revealTarget.set(null);
  }

  $: show(path);
  $: reveal($revealTarget, current);
</script>

<div class="host" bind:this={host}></div>

<style>
  .host {
    flex: 1;
    min-height: 0;
  }
</style>
