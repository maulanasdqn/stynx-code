<script>
  import Icon from "../icon.svelte";
  import { info, status } from "../../lib/stores.js";
  import { recentWorkspaces, forgetWorkspace } from "../../lib/workspaces.js";
  import { pickFolder, popupMenu, confirmDestructive } from "../../lib/native.js";

  export let onOpenWorkspace;

  let recents = recentWorkspaces();

  $: if ($info) recents = recentWorkspaces();
  $: current = $info?.workspacePath;

  const lastComponent = (path) => path.split("/").filter(Boolean).pop() ?? path;

  async function addWorkspace() {
    try {
      const path = await pickFolder();
      if (path) onOpenWorkspace(path, null);
    } catch (error) {
      $status = `Open panel failed: ${error}`;
    }
  }

  async function remove(path) {
    const ok = await confirmDestructive(
      `Remove "${lastComponent(path)}" from workspace list?`,
      "This only removes it from the list. The folder on disk is not deleted.",
      "Remove",
    );
    if (ok) recents = forgetWorkspace(path);
  }

  function contextMenu(event, path) {
    event.preventDefault();
    popupMenu(
      [
        { text: "Open", action: () => onOpenWorkspace(path, null) },
        { text: "Remove from list", action: () => remove(path) },
      ],
      event,
    );
  }
</script>

<section>
  <div class="sb-header">Workspace</div>
  {#each recents as path (path)}
    <button
      class="sb-row"
      title={path === current ? `${path} · running` : path}
      on:click={() => onOpenWorkspace(path, null)}
      on:contextmenu={(event) => contextMenu(event, path)}
    >
      <span class="folder" class:current={path === current}>
        <Icon name={path === current ? "folder.fill" : "folder"} size={13} />
      </span>
      <span class="name ellipsis" class:current={path === current}>{lastComponent(path)}</span>
      {#if path === current}<span class="dot"></span>{/if}
    </button>
  {/each}
  <button class="sb-row" on:click={addWorkspace}>
    <span class="sb-label-icon"><Icon name="plus.rectangle.on.folder" size={15} /></span>
    Add workspace
  </button>
</section>

<style>
  .folder {
    color: var(--secondary);
    width: 16px;
    display: flex;
    justify-content: center;
  }

  .folder.current {
    color: var(--accent);
  }

  .name {
    flex: 1;
    min-width: 0;
  }

  .name.current {
    font-weight: 600;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
  }
</style>
