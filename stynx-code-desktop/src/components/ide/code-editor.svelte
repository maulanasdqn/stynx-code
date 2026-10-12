<script>
  import { onDestroy, onMount } from "svelte";
  import { monaco } from "../../lib/ide/monaco-setup.js";
  import { modelFor } from "../../lib/ide/documents.js";
  import { editorContext, revealTarget } from "../../lib/ide/ide-stores.js";
  import { askAgent } from "../../lib/ide/agent-context.js";

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
    editor.onDidChangeCursorSelection(updateContext);
    editor.addAction({
      id: "stynx.ask",
      label: "Ask Stynx",
      keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyL],
      contextMenuGroupId: "navigation",
      contextMenuOrder: 0,
      run: () => {
        updateContext();
        askAgent();
      },
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
    updateContext();
    if (model) {
      model.tokenization?.forceTokenization?.(Math.min(model.getLineCount(), 300));
      const state = viewStates.get(next);
      if (state) editor.restoreViewState(state);
      editor.focus();
    }
  }

  function updateContext() {
    const model = editor?.getModel();
    if (!model || !current) {
      editorContext.set(null);
      return;
    }
    const selection = editor.getSelection();
    const text = selection && !selection.isEmpty() ? model.getValueInRange(selection) : "";
    editorContext.set({
      path: current,
      line: editor.getPosition()?.lineNumber ?? null,
      startLine: selection?.startLineNumber ?? null,
      endLine: selection?.endLineNumber ?? null,
      text,
    });
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
