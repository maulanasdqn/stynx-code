<script>
  import Icon from "../icon.svelte";
  import { references } from "../../lib/stores.js";
  import { addReferencePaths, removeReference } from "../../lib/references.js";
  import { pickFiles } from "../../lib/native.js";

  async function addReference() {
    addReferencePaths(await pickFiles({ title: "Choose reference documents" }));
  }

  function iconFor(name) {
    const ext = name.split(".").pop().toLowerCase();
    if (ext === "pdf") return "doc.richtext";
    if (ext === "doc" || ext === "docx") return "doc.text";
    if (ext === "md" || ext === "txt") return "doc.plaintext";
    return "doc";
  }
</script>

<section>
  <div class="sb-header">References</div>
  <button class="sb-row" on:click={addReference}>
    <span class="sb-label-icon"><Icon name="doc.badge.plus" size={15} /></span>
    Add reference
  </button>
  {#each $references as doc (doc.id)}
    <div class="sb-row" title={doc.path}>
      <span class="glyph" class:has-text={doc.text}><Icon name={iconFor(doc.name)} size={13} /></span>
      <span class="sb-meta">
        <span class="sb-title ellipsis">{doc.name}</span>
        <span class="sb-sub">{doc.text ? "reference" : "no text extracted"}</span>
      </span>
      <button class="remove" title="Remove" on:click={() => removeReference(doc.id)}>
        <Icon name="xmark.circle.fill" size={14} />
      </button>
    </div>
  {:else}
    <div class="sb-empty">No reference documents</div>
  {/each}
</section>

<style>
  .glyph {
    color: var(--secondary);
    width: 16px;
    display: flex;
    justify-content: center;
  }

  .glyph.has-text {
    color: var(--accent);
  }

  .remove {
    color: var(--secondary);
    display: flex;
  }
</style>
