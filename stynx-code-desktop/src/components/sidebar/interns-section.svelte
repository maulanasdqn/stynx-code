<script>
  import Icon from "../icon.svelte";
  import KeySheet from "../key-sheet.svelte";
  import { interns } from "../../lib/stores.js";
  import { saveInternKey } from "../../lib/session-actions.js";

  let keyEntry = null;

  async function save(value) {
    const entry = keyEntry;
    keyEntry = null;
    await saveInternKey(entry.keyEnv, value);
  }
</script>

<section>
  <div class="sb-header">Interns</div>
  {#each $interns as intern (intern.name)}
    <div class="sb-row intern">
      <span class="glyph" class:ready={intern.available}>
        <Icon name={intern.available ? "person.fill.checkmark" : "person.slash"} size={13} />
      </span>
      <span class="sb-meta">
        <span class="head">
          <span class="name mono">{intern.name}</span>
          {#if intern.available}
            <span class="ready-label">ready</span>
          {:else}
            <button class="add-key" title="Set {intern.keyEnv}" on:click={() => (keyEntry = intern)}>
              <Icon name="key" size={12} />
            </button>
          {/if}
        </span>
        <span class="description">{intern.description}</span>
        <span class="sb-sub">{intern.provider} · {intern.model}</span>
      </span>
    </div>
  {:else}
    <div class="sb-empty">No interns configured</div>
  {/each}
</section>

{#if keyEntry}
  <KeySheet entry={keyEntry} onSave={save} onCancel={() => (keyEntry = null)} />
{/if}

<style>
  .intern {
    align-items: flex-start;
    padding-top: 4px;
    padding-bottom: 4px;
  }

  .glyph {
    color: var(--secondary);
    width: 16px;
    display: flex;
    justify-content: center;
    padding-top: 2px;
  }

  .glyph.ready {
    color: var(--green);
  }

  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
  }

  .name {
    font-size: var(--callout);
  }

  .ready-label {
    font-size: var(--caption);
    color: var(--green);
  }

  .add-key {
    color: var(--blue);
    display: flex;
  }

  .description {
    font-size: var(--caption);
    color: var(--secondary);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
