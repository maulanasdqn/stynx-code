<script>
  import Icon from "./icon.svelte";
  import { highlight } from "../lib/highlight.js";

  export let change;
  export let selected;
  export let onToggle;

  const SIGN = { added: "+", removed: "-", context: "" };
</script>

<div class="lines">
  {#if change.diff}
    {#each change.diff as line (line.id)}
      <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
      <div class="row {line.kind}" class:selected={selected.has(line.id)} on:click={() => onToggle(line.id)}>
        <span class="check" class:on={selected.has(line.id)}>
          <Icon name={selected.has(line.id) ? "checkmark.square.fill" : "square"} size={10} weight={2} />
        </span>
        <span class="sign">{SIGN[line.kind]}</span>
        <span class="code">{@html highlight(line.text)}</span>
      </div>
    {/each}
  {:else if change.lines}
    {#each change.lines as text, index}
      <div class="row numbered">
        <span class="number">{index + 1}</span>
        <span class="code">{@html highlight(text)}</span>
      </div>
    {/each}
  {/if}
</div>

<style>
  .lines {
    padding: 6px 0;
    font-family: var(--mono);
    font-size: var(--callout);
    overflow-x: auto;
  }

  .row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 1px 12px;
    min-width: max-content;
  }

  .row.added {
    background: color-mix(in srgb, var(--green) 12%, transparent);
  }

  .row.removed {
    background: color-mix(in srgb, var(--red) 12%, transparent);
  }

  .row.selected {
    background: rgba(var(--accent-rgb), 0.18);
  }

  .row.numbered {
    gap: 12px;
  }

  .check {
    width: 14px;
    padding-top: 3px;
    display: flex;
    justify-content: center;
    color: color-mix(in srgb, var(--secondary) 40%, transparent);
  }

  .check.on {
    color: var(--accent);
  }

  .sign {
    width: 12px;
    text-align: center;
    flex-shrink: 0;
  }

  .added .sign {
    color: var(--green);
  }

  .removed .sign {
    color: var(--red);
  }

  .number {
    width: 34px;
    text-align: right;
    flex-shrink: 0;
    font-size: var(--caption);
    line-height: 16px;
    color: var(--tertiary);
  }

  .code {
    white-space: pre;
    color: var(--label);
    user-select: text;
    -webkit-user-select: text;
  }

  .code :global(.hl-comment) {
    color: var(--secondary);
  }

  .code :global(.hl-num) {
    color: rgb(128, 204, 230);
  }

  .code :global(.hl-kw) {
    color: rgb(217, 128, 217);
  }

  .code :global(.hl-str) {
    color: rgb(230, 153, 102);
  }
</style>
