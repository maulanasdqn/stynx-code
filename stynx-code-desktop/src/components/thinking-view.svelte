<script>
  import Icon from "./icon.svelte";
  import Markdown from "./markdown.svelte";

  export let text;

  let enlarged = false;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div class="thinking" on:click={() => (enlarged = !enlarged)}>
  <div class="head">
    <Icon name="brain" size={12} />
    <span>Thinking</span>
    <span class="spacer"></span>
    <span class="chevron" class:open={enlarged}><Icon name="chevron.down" size={9} weight={2.4} /></span>
  </div>
  <div class="content" class:collapsed={!enlarged}>
    <Markdown raw={text} size="callout" dim />
  </div>
</div>

<style>
  .thinking {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 10px;
    border-radius: 8px;
    background: rgba(var(--fg-rgb), 0.035);
    border: 1px solid rgba(var(--fg-rgb), 0.07);
  }

  .head {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: var(--caption);
    font-weight: 600;
    color: var(--secondary);
  }

  .spacer {
    flex: 1;
  }

  .chevron {
    display: flex;
    transition: transform 0.5s var(--spring);
  }

  .chevron.open {
    transform: rotate(180deg);
  }

  .content {
    max-height: 4000px;
    transition: max-height 0.6s var(--smooth);
  }

  .content.collapsed {
    max-height: 360px;
    overflow: hidden;
    -webkit-mask-image: linear-gradient(to bottom, black 0%, black 82%, transparent 100%);
    mask-image: linear-gradient(to bottom, black 0%, black 82%, transparent 100%);
  }
</style>
