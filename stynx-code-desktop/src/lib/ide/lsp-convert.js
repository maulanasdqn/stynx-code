import { monaco } from "./monaco-setup.js";

const K = monaco.languages.CompletionItemKind;
const KINDS = [K.Text, K.Text, K.Method, K.Function, K.Constructor, K.Field, K.Variable, K.Class, K.Interface,
  K.Module, K.Property, K.Unit, K.Value, K.Enum, K.Keyword, K.Snippet, K.Color, K.File, K.Reference, K.Folder,
  K.EnumMember, K.Constant, K.Struct, K.Event, K.Operator, K.TypeParameter];

export const toRange = (r) => new monaco.Range(r.start.line + 1, r.start.character + 1, r.end.line + 1, r.end.character + 1);

function docText(doc) {
  if (!doc) return undefined;
  if (typeof doc === "string") return doc;
  return { value: doc.value ?? "" };
}

export function toCompletion(item, fallbackRange) {
  const edit = item.textEdit;
  const range = edit?.range ? toRange(edit.range) : edit?.insert ? { insert: toRange(edit.insert), replace: toRange(edit.replace) } : fallbackRange;
  return {
    label: item.labelDetails?.detail ? { label: item.label, detail: item.labelDetails.detail, description: item.labelDetails.description } : item.label,
    kind: KINDS[item.kind] ?? K.Text,
    detail: item.detail,
    documentation: docText(item.documentation),
    insertText: edit?.newText ?? item.insertText ?? item.label,
    insertTextRules: item.insertTextFormat === 2 ? monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet : undefined,
    range,
    sortText: item.sortText,
    filterText: item.filterText,
    preselect: item.preselect,
    commitCharacters: item.commitCharacters,
    additionalTextEdits: (item.additionalTextEdits ?? []).map((e) => ({ range: toRange(e.range), text: e.newText })),
    lspItem: item,
  };
}

export function toHoverContents(contents) {
  const list = Array.isArray(contents) ? contents : [contents];
  return list
    .filter(Boolean)
    .map((c) => (typeof c === "string" ? { value: c } : c.language ? { value: "```" + c.language + "\n" + c.value + "\n```" } : { value: c.value ?? "" }));
}

const SEVERITY = [null, "Error", "Warning", "Info", "Hint"];

export function toMarkers(list) {
  return list.map((d) => ({
    severity: monaco.MarkerSeverity[SEVERITY[d.severity ?? 1]] ?? monaco.MarkerSeverity.Error,
    message: d.message,
    source: d.source,
    code: typeof d.code === "object" ? d.code?.value : d.code != null ? String(d.code) : undefined,
    startLineNumber: d.range.start.line + 1,
    startColumn: d.range.start.character + 1,
    endLineNumber: d.range.end.line + 1,
    endColumn: d.range.end.character + 1,
  }));
}
