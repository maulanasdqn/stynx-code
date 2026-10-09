<script>
  import Icon from "./icon.svelte";
  import WorkspaceSection from "./sidebar/workspace-section.svelte";
  import SessionsSection from "./sidebar/sessions-section.svelte";
  import ModelSection from "./sidebar/model-section.svelte";
  import PermissionSection from "./sidebar/permission-section.svelte";
  import ReferencesSection from "./sidebar/references-section.svelte";
  import InternsSection from "./sidebar/interns-section.svelte";
  import { showSidebar } from "../lib/stores.js";
  import { isMac, startWindowDrag } from "../lib/native.js";

  export let onOpenWorkspace;
</script>

<aside>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="chrome" class:mac={isMac} on:mousedown={startWindowDrag}>
    <button class="toggle" title="Hide sidebar" on:click={() => ($showSidebar = false)}>
      <Icon name="sidebar.left" size={17} weight={1.6} />
    </button>
  </div>
  <div class="list">
    <WorkspaceSection {onOpenWorkspace} />
    <SessionsSection />
    <ModelSection {onOpenWorkspace} />
    <PermissionSection />
    <ReferencesSection />
    <InternsSection />
  </div>
</aside>

<style>
  aside {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .chrome {
    height: var(--toolbar-h);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    padding: 0 10px;
  }

  .toggle {
    width: 34px;
    height: 30px;
    border-radius: 999px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--secondary);
  }

  .toggle:hover {
    background: var(--hover);
    color: var(--label);
  }

  .list {
    flex: 1;
    overflow-y: auto;
    padding: 0 10px 16px;
  }

  .list :global(section) {
    margin-bottom: 10px;
  }

  .list :global(.sb-header) {
    font-size: var(--subheadline);
    font-weight: 700;
    color: var(--secondary);
    opacity: 0.85;
    padding: 6px 8px 4px;
  }

  .list :global(.sb-row) {
    width: 100%;
    min-height: 26px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 3px 8px;
    border-radius: 6px;
    text-align: left;
    font-size: var(--body);
    transition:
      background-color 0.3s var(--smooth),
      transform 0.45s var(--spring);
  }

  .list :global(button.sb-row:hover:not(:disabled)) {
    background: var(--hover);
    transform: translateX(2px);
  }

  .list :global(button.sb-row:active:not(:disabled)) {
    transform: scale(0.97);
  }

  .list :global(.sb-label-icon) {
    color: var(--accent);
    width: 16px;
    display: flex;
    justify-content: center;
  }

  .list :global(.sb-empty) {
    font-size: var(--callout);
    color: var(--tertiary);
    padding: 3px 8px;
  }

  .list :global(.sb-meta) {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
    flex: 1;
  }

  .list :global(.sb-title) {
    font-size: var(--callout);
  }

  .list :global(.sb-sub) {
    font-size: var(--caption);
    color: var(--tertiary);
  }

  .list :global(.sb-value) {
    margin-left: auto;
    color: var(--secondary);
    text-align: right;
  }

  .list :global(select) {
    margin-left: auto;
    max-width: 150px;
    font: inherit;
    font-size: var(--body);
  }
</style>
