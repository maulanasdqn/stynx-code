<script>
  import Icon from "./icon.svelte";
  import { pendingImages } from "../lib/stores.js";
  import { removeImage, dataUrl } from "../lib/images.js";
</script>

{#if $pendingImages.length > 0}
  <div class="strip">
    {#each $pendingImages as image (image.id)}
      <div class="thumb">
        <img src={dataUrl(image)} alt="attachment" />
        <button on:click={() => removeImage(image.id)} title="Remove">
          <Icon name="xmark.circle.fill" size={14} />
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .strip {
    display: flex;
    gap: 8px;
    height: 62px;
    align-items: flex-end;
    overflow-x: auto;
    padding: 0 6px 0 2px;
  }

  .strip::-webkit-scrollbar {
    display: none;
  }

  .thumb {
    position: relative;
    flex-shrink: 0;
  }

  .thumb img {
    width: 54px;
    height: 54px;
    object-fit: cover;
    border-radius: 8px;
    display: block;
  }

  .thumb button {
    position: absolute;
    top: -5px;
    right: -5px;
    display: flex;
    color: rgba(0, 0, 0, 0.6);
    background: radial-gradient(circle, white 40%, transparent 42%);
    border-radius: 50%;
  }
</style>
