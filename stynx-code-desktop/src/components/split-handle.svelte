<script>
  export let onDrag;
  export let line = true;
  export let vertical = false;

  let start = 0;

  const point = (event) => (vertical ? event.clientY : event.clientX);

  function down(event) {
    start = point(event);
    event.currentTarget.setPointerCapture(event.pointerId);
  }

  function move(event) {
    if (!event.currentTarget.hasPointerCapture(event.pointerId)) return;
    onDrag(point(event) - start);
    start = point(event);
  }
</script>

<div
  class="handle"
  class:line
  class:vertical
  role="separator"
  aria-orientation={vertical ? "horizontal" : "vertical"}
  on:pointerdown={down}
  on:pointermove={move}
></div>

<style>
  .handle {
    width: 7px;
    margin: 0 -3px;
    flex-shrink: 0;
    cursor: col-resize;
    position: relative;
    z-index: 5;
  }

  .handle.vertical {
    width: auto;
    height: 7px;
    margin: -3px 0;
    cursor: row-resize;
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
