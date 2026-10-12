<script>
  import { onDestroy, onMount } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import { WebLinksAddon } from "@xterm/addon-web-links";
  import "@xterm/xterm/css/xterm.css";
  import { resize, subscribe, write } from "../../lib/ide/terminals.js";
  import { terminalTheme } from "../../lib/ide/terminal-theme.js";
  import { openExternal } from "../../lib/native.js";

  export let key;
  export let active = true;

  let host;
  let term;
  let fit;
  let unsubscribe = () => {};
  let observer;
  const scheme = window.matchMedia("(prefers-color-scheme: dark)");

  function refit() {
    if (!term || !active || !host?.offsetWidth) return;
    fit.fit();
    resize(key, term.cols, term.rows);
  }

  const retheme = () => term && (term.options.theme = terminalTheme());

  onMount(() => {
    term = new Terminal({
      fontFamily: 'ui-monospace, "SF Mono", Menlo, monospace',
      fontSize: 12.5,
      lineHeight: 1.15,
      cursorBlink: true,
      cursorStyle: "bar",
      allowProposedApi: true,
      scrollback: 5000,
      macOptionIsMeta: true,
      theme: terminalTheme(),
    });
    fit = new FitAddon();
    term.loadAddon(fit);
    term.loadAddon(new WebLinksAddon((_event, uri) => openExternal(uri)));
    term.open(host);
    term.onData((data) => write(key, data));
    unsubscribe = subscribe(key, (data) => term.write(data));
    observer = new ResizeObserver(() => requestAnimationFrame(refit));
    observer.observe(host);
    scheme.addEventListener("change", retheme);
    refit();
    term.focus();
  });

  onDestroy(() => {
    unsubscribe();
    observer?.disconnect();
    scheme.removeEventListener("change", retheme);
    term?.dispose();
  });

  $: if (active && term) {
    requestAnimationFrame(() => {
      refit();
      term.focus();
    });
  }
</script>

<div class="term" class:hidden={!active} bind:this={host}></div>

<style>
  .term {
    position: absolute;
    inset: 4px 0 0 10px;
  }

  .term.hidden {
    visibility: hidden;
  }

  .term :global(.xterm-viewport) {
    background: transparent !important;
  }
</style>
