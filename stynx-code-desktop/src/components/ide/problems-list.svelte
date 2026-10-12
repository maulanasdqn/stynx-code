<script>
  import Icon from "../icon.svelte";
  import { diagnostics, revealTarget } from "../../lib/ide/ide-stores.js";
  import { openFile } from "../../lib/ide/documents.js";

  const SEVERITY = { 1: "error", 2: "warning", 3: "info", 4: "info" };

  $: files = [...$diagnostics.entries()]
    .map(([path, list]) => ({ path, list: [...list].sort((a, b) => (a.severity ?? 1) - (b.severity ?? 1) || a.range.start.line - b.range.start.line) }))
    .sort((a, b) => a.path.localeCompare(b.path));

  async function jump(path, diagnostic) {
    await openFile(path);
    revealTarget.set({ path, line: diagnostic.range.start.line + 1, column: diagnostic.range.start.character + 1 });
  }
</script>

<div class="problems">
  <header class="head"><span class="title">Problems</span></header>
  <div class="list">
    {#each files as file (file.path)}
      <div class="file">
        <Icon name="doc" size={12} />
        <span class="name ellipsis">{file.path.split("/").pop()}</span>
        <span class="dir ellipsis">{file.path}</span>
        <span class="count">{file.list.length}</span>
      </div>
      {#each file.list as diagnostic, index (index)}
        {@const level = SEVERITY[diagnostic.severity ?? 1]}
        <button class="item" on:click={() => jump(file.path, diagnostic)}>
          <span class="icon {level}">
            <Icon name={level === "info" ? "circle.fill" : "exclamationmark.triangle.fill"} size={level === "info" ? 8 : 11} />
          </span>
          <span class="message">{diagnostic.message}</span>
          <span class="where">{diagnostic.range.start.line + 1}:{diagnostic.range.start.character + 1}</span>
        </button>
      {/each}
    {:else}
      <div class="empty">No problems detected.</div>
    {/each}
  </div>
</div>

<style>
  .problems { flex: 1; min-height: 0; display: flex; flex-direction: column; }
  .head { padding: 10px 8px 6px 14px; }
  .title {
    font-size: var(--subheadline);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: var(--secondary);
  }
  .list { flex: 1; overflow: auto; padding: 0 6px 12px; }
  .file { display: flex; align-items: center; gap: 6px; padding: 8px 8px 4px; color: var(--secondary); }
  .name { color: var(--label); font-size: var(--body); flex-shrink: 0; max-width: 55%; }
  .dir { flex: 1; min-width: 0; font-size: var(--caption); color: var(--tertiary); }
  .count { font-size: var(--caption); color: var(--tertiary); }
  .item {
    width: 100%;
    display: flex;
    align-items: flex-start;
    gap: 7px;
    padding: 4px 8px 4px 22px;
    border-radius: 6px;
    text-align: left;
    font-size: var(--callout);
  }
  .item:hover { background: var(--hover); }
  .icon { display: flex; padding-top: 2px; }
  .icon.error { color: var(--red); }
  .icon.warning { color: var(--orange); }
  .icon.info { color: var(--blue); padding-top: 4px; }
  .message {
    flex: 1;
    min-width: 0;
    color: var(--label);
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .where { font-family: var(--mono); font-size: var(--caption); color: var(--tertiary); }
  .empty { padding: 10px 14px; font-size: var(--callout); color: var(--tertiary); }
</style>
