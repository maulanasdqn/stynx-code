import { get } from "svelte/store";
import { references, status } from "./stores.js";
import { fetchReference, readFile } from "./api.js";

const MAX_CHARS = 60_000;
let counter = 0;
let dirty = false;

export async function addReferencePaths(paths) {
  for (const path of paths) {
    const name = path.split("/").pop() ?? path;
    let text = "";
    try {
      text = (await readFile(path)).slice(0, MAX_CHARS);
    } catch {
    }
    push(name, path, looksBinary(text) ? "" : text);
  }
}

export async function addReferenceFromUrl(link) {
  const trimmed = link.trim();
  if (!/^https?:\/\/\S+$/.test(trimmed)) return false;
  const name = trimmed.split("/").filter(Boolean).pop() ?? trimmed;
  status.set(`Fetching ${name}…`);
  try {
    const result = await fetchReference(trimmed);
    const text = result.contentType.includes("html") ? stripHtml(result.body) : result.body;
    push(name, trimmed, text.slice(0, MAX_CHARS));
  } catch {
    status.set("Fetch failed");
    return true;
  }
  if (get(status).startsWith("Fetching")) status.set("Ready");
  return true;
}

export function removeReference(id) {
  references.update((docs) => docs.filter((doc) => doc.id !== id));
  dirty = true;
}

export function injectReferences(text) {
  const docs = get(references);
  if (docs.length === 0 || !dirty) return { text, count: 0 };
  dirty = false;
  const blocks = docs
    .map((doc) => `## ${doc.name}\n${doc.text || `(no extractable text — ${doc.path})`}`)
    .join("\n\n");
  const intro =
    "The user attached the following reference document(s). Treat their content as authoritative context for this and following requests.";
  return {
    text: `${intro}\n\n<reference_documents>\n${blocks}\n</reference_documents>\n\n${text}`,
    count: docs.length,
  };
}

function push(name, path, text) {
  references.update((docs) => [...docs, { id: `ref-${++counter}`, name, path, text }]);
  dirty = true;
}

function looksBinary(text) {
  return text.includes("\uFFFD");
}

function stripHtml(html) {
  let s = html;
  s = s.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, " ");
  s = s.replace(/<style[^>]*>[\s\S]*?<\/style>/gi, " ");
  s = s.replace(/<[^>]+>/g, " ");
  s = s
    .replaceAll("&nbsp;", " ")
    .replaceAll("&amp;", "&")
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replaceAll("&#39;", "'")
    .replaceAll("&quot;", '"');
  s = s.replace(/[ \t]+/g, " ");
  s = s.replace(/\n{3,}/g, "\n\n");
  return s.trim();
}
