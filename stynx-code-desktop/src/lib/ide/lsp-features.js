import { monaco } from "./monaco-setup.js";
import { lspRequest } from "./ide-api.js";
import { flush, pathOf } from "./lsp-sync.js";
import { toRange } from "./lsp-convert.js";

const K = monaco.languages.SymbolKind;
const SYMBOL_KINDS = [K.File, K.File, K.Module, K.Namespace, K.Package, K.Class, K.Method, K.Property, K.Field, K.Constructor,
  K.Enum, K.Interface, K.Function, K.Variable, K.Constant, K.String, K.Number, K.Boolean, K.Array, K.Object, K.Key, K.Null,
  K.EnumMember, K.Struct, K.Event, K.Operator, K.TypeParameter];

async function request(model, method, params) {
  const path = pathOf(model);
  if (!path) return null;
  await flush(path);
  return lspRequest(path, method, params).catch(() => null);
}

function toSymbol(symbol) {
  const range = toRange(symbol.range ?? symbol.location.range);
  return {
    name: symbol.name,
    detail: symbol.detail ?? "",
    kind: SYMBOL_KINDS[symbol.kind] ?? K.Variable,
    tags: [],
    range,
    selectionRange: symbol.selectionRange ? toRange(symbol.selectionRange) : range,
    children: (symbol.children ?? []).map(toSymbol),
  };
}

const docText = (doc) => (doc ? (typeof doc === "string" ? doc : { value: doc.value ?? "" }) : undefined);

export function registerLspFeatures() {
  monaco.languages.registerDocumentFormattingEditProvider("*", {
    async provideDocumentFormattingEdits(model, options) {
      const edits = await request(model, "textDocument/formatting", { options: { tabSize: options.tabSize, insertSpaces: options.insertSpaces } });
      return (edits ?? []).map((edit) => ({ range: toRange(edit.range), text: edit.newText }));
    },
  });
  monaco.languages.registerDocumentSymbolProvider("*", {
    async provideDocumentSymbols(model) {
      const symbols = await request(model, "textDocument/documentSymbol", {});
      return (symbols ?? []).map(toSymbol);
    },
  });
  monaco.languages.registerSignatureHelpProvider("*", {
    signatureHelpTriggerCharacters: ["(", ","],
    signatureHelpRetriggerCharacters: [")"],
    async provideSignatureHelp(model, position) {
      const help = await request(model, "textDocument/signatureHelp", { position: { line: position.lineNumber - 1, character: position.column - 1 } });
      if (!help?.signatures?.length) return null;
      const signatures = help.signatures.map((sig) => ({
        label: sig.label,
        documentation: docText(sig.documentation),
        parameters: (sig.parameters ?? []).map((p) => ({ label: p.label, documentation: docText(p.documentation) })),
      }));
      const value = { signatures, activeSignature: help.activeSignature ?? 0, activeParameter: help.activeParameter ?? 0 };
      return { value, dispose() {} };
    },
  });
}
