<script>
  import { parseMarkdown } from "../lib/markdown.js";

  export let raw;
  export let size = "body";
  export let dim = false;

  $: blocks = parseMarkdown(raw);
</script>

<div class="markdown selectable {size}" class:dim>
  {#each blocks as block}
    {#if block.kind === "code"}
      <pre>{block.text}</pre>
    {:else if block.kind === "heading"}
      <div class="heading" class:big={block.level <= 2}>{@html block.html}</div>
    {:else if block.kind === "bullet"}
      <div class="listrow"><span class="marker">•</span><span class="body">{@html block.html}</span></div>
    {:else if block.kind === "numbered"}
      <div class="listrow"><span class="marker">{block.number}.</span><span class="body">{@html block.html}</span></div>
    {:else if block.kind === "rule"}
      <hr />
    {:else if block.kind === "table"}
      <div class="table-wrap">
        <table>
          <thead>
            <tr>{#each block.header as cell}<th>{@html cell}</th>{/each}</tr>
          </thead>
          <tbody>
            {#each block.rows as row}
              <tr>{#each row as cell}<td>{@html cell}</td>{/each}</tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else if block.kind === "blank"}
      <div class="blank"></div>
    {:else}
      <div class="text">{@html block.html}</div>
    {/if}
  {/each}
</div>

<style>
  .markdown {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: var(--body);
    line-height: 1.45;
    min-width: 0;
  }

  .markdown.callout {
    font-size: var(--callout);
  }

  .markdown.dim {
    color: var(--secondary);
  }

  .text,
  .body {
    white-space: pre-wrap;
    word-break: break-word;
  }

  .heading {
    font-size: var(--headline);
    font-weight: 600;
  }

  .heading.big {
    font-size: var(--title3);
  }

  .listrow {
    display: flex;
    gap: 8px;
  }

  .marker {
    color: var(--secondary);
    flex-shrink: 0;
    font-variant-numeric: tabular-nums;
  }

  .body {
    flex: 1;
    min-width: 0;
  }

  hr {
    border: none;
    border-top: 1px solid var(--separator);
    margin: 4px 0;
  }

  .blank {
    height: 2px;
  }

  .markdown :global(code) {
    font-family: var(--mono);
    font-size: 0.95em;
  }

  .markdown :global(a) {
    color: var(--accent);
    text-decoration: none;
  }

  pre,
  .table-wrap {
    background: var(--text-bg);
    border: 1px solid color-mix(in srgb, var(--secondary) 18%, transparent);
    border-radius: 8px;
    padding: 10px;
  }

  pre {
    font-family: var(--mono);
    font-size: var(--callout);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .table-wrap {
    overflow-x: auto;
  }

  table {
    border-collapse: collapse;
    font-size: var(--callout);
  }

  th,
  td {
    text-align: left;
    vertical-align: top;
    padding: 3px 16px 3px 0;
  }

  th {
    font-weight: 600;
    border-bottom: 1px solid var(--separator);
    padding-bottom: 6px;
  }

  tbody tr:first-child td {
    padding-top: 6px;
  }
</style>
