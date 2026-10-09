<script>
  export let width = 460;
  export let onCancel;

  function keydown(event) {
    if (event.key === "Escape") onCancel();
  }
</script>

<svelte:window on:keydown={keydown} />

<div class="scrim">
  <div class="sheet glass panel" style="width: {width}px">
    <slot />
  </div>
</div>

<style>
  .scrim {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top: calc(var(--toolbar-h) + 4px);
    background: rgba(0, 0, 0, 0.12);
  }

  .sheet {
    padding: 20px;
    border-radius: 24px;
    background:
      linear-gradient(180deg, var(--glass-tint), var(--glass-tint-2)),
      var(--material-regular);
    backdrop-filter: blur(40px) saturate(1.9);
    -webkit-backdrop-filter: blur(40px) saturate(1.9);
    animation: drop 0.6s var(--spring);
  }

  .scrim {
    animation: fade 0.3s var(--smooth);
  }

  @keyframes drop {
    from {
      transform: translateY(-40px) scaleY(0.92);
      transform-origin: top;
      opacity: 0;
      filter: blur(6px);
    }
  }

  @keyframes fade {
    from {
      background: transparent;
    }
  }
</style>
