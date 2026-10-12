import "monaco-editor/esm/vs/editor/edcore.main.js";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker&inline";
import { applyTheme, watchColorScheme } from "./monaco-theme.js";
import { restoreInstalled } from "./syntax.js";

let ready = false;

export function initMonaco() {
  if (!ready) {
    self.MonacoEnvironment = { getWorker: () => new EditorWorker() };
    applyTheme();
    watchColorScheme();
    ready = true;
    restoreInstalled();
  }
  return monaco;
}

export { monaco };
