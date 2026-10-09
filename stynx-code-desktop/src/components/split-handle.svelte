<script>
  export let onDrag;
  export let line = true;

  let startX = 0;

  function down(event) {
    startX = event.clientX;
    event.currentTarget.setPointerCapture(event.pointerId);
  }

  function move(event) {
    if (!event.currentTarget.hasPointerCapture(event.pointerId)) return;
    onDrag(event.clientX - startX);
    startX = event.clientX;
  }
</script>

<div class="handle" class:line role="separator" aria-orientation="vertical" on:pointerdown={down} on:pointermove={move}></div>

<style>
  .handle {
    width: 7px;
    margin: 0 -3px;
    flex-shrink: 0;
    cursor: col-resize;
    position: relative;
    z-index: 5;
  }

  .handle:not(.line) {
    background: var(--text-bg);
  }

  .handle.line::after {
    content: "";
    position: absolute;
    top: 0;
    bottom: 0;
    left: 3px;
    width: 1px;
    background: var(--separator);
  }
</style>
