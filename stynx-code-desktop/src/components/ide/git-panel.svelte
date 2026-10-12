<script>
  import { onMount } from "svelte";
  import Icon from "../icon.svelte";
  import GitFileRow from "./git-file-row.svelte";
  import { commitMessage, gitBusy, gitState } from "../../lib/ide/ide-stores.js";
  import { commit, discard, refreshGit, stage, unstage } from "../../lib/ide/git.js";

  onMount(() => refreshGit());

  $: s = $gitState;
  $: canCommit = !$gitBusy && s.staged.length > 0 && $commitMessage.trim().length > 0;
  $: sections = [
    { id: "conflicted", title: "Merge Conflicts", entries: s.conflicted },
    { id: "staged", title: "Staged Changes", entries: s.staged },
    { id: "unstaged", title: "Changes", entries: s.unstaged },
  ].filter((section) => section.entries.length > 0);

  const paths = (entries) => entries.map((entry) => entry.path);

  function keydown(event) {
    if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      commit();
    }
  }
</script>

<div class="git">
  <header class="head">
    <span class="title">Source Control</span>
    <button class="tool" title="Refresh" on:click={() => refreshGit()}><Icon name="arrow.clockwise" size={14} /></button>
  </header>
  {#if !s.isRepo}
    <div class="empty">This workspace isn't a Git repository.</div>
  {:else}
    <div class="branch">
      <Icon name="arrow.triangle.branch" size={13} />
      <span class="ellipsis">{s.detached ? "detached HEAD" : s.branch ?? "no branch"}</span>
      {#if s.ahead}<span class="sync">↑{s.ahead}</span>{/if}
      {#if s.behind}<span class="sync">↓{s.behind}</span>{/if}
    </div>
    <div class="commit">
      <textarea class="field" rows="2" placeholder="Message (⌘↵ to commit)" bind:value={$commitMessage} on:keydown={keydown}></textarea>
      <button class="push prominent" disabled={!canCommit} on:click={commit}>
        {s.staged.length ? `Commit ${s.staged.length} file${s.staged.length === 1 ? "" : "s"}` : "Commit"}
      </button>
    </div>
    <div class="list">
      {#each sections as section (section.id)}
        <div class="section">
          <span class="section-title">{section.title}</span>
          <span class="count">{section.entries.length}</span>
          <span class="spacer"></span>
          {#if section.id === "staged"}
            <button class="tool small" title="Unstage All" on:click={() => unstage(paths(section.entries))}><Icon name="xmark" size={10} weight={2.4} /></button>
          {:else if section.id === "unstaged"}
            <button class="tool small" title="Discard All" on:click={() => discard(paths(section.entries))}><Icon name="arrow.counterclockwise" size={12} /></button>
            <button class="tool small" title="Stage All" on:click={() => stage(paths(section.entries))}><Icon name="plus" size={12} weight={2.2} /></button>
          {/if}
        </div>
        {#each section.entries as entry (entry.path + section.id)}
          <GitFileRow {entry} staged={section.id === "staged"} conflicted={section.id === "conflicted"} />
        {/each}
      {:else}
        <div class="empty">No changes.</div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .git { flex: 1; min-height: 0; display: flex; flex-direction: column; }
  .head { display: flex; align-items: center; padding: 10px 8px 6px 14px; }
  .title, .section-title {
    font-size: var(--subheadline);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: var(--secondary);
  }
  .title { flex: 1; }
  .branch { display: flex; align-items: center; gap: 6px; padding: 2px 14px 8px; font-size: var(--callout); color: var(--label); }
  .sync { font-size: var(--caption); color: var(--secondary); font-variant-numeric: tabular-nums; }
  .commit { display: flex; flex-direction: column; gap: 6px; padding: 0 10px 10px; }
  textarea { resize: vertical; min-height: 44px; background: var(--text-bg); }
  .list { flex: 1; overflow: auto; padding: 0 6px 12px; }
  .section { display: flex; align-items: center; gap: 6px; padding: 10px 8px 4px 8px; }
  .count { font-size: var(--caption); color: var(--tertiary); }
  .spacer { flex: 1; }
  .tool {
    width: 24px; height: 24px; border-radius: 6px;
    display: flex; align-items: center; justify-content: center; color: var(--secondary);
  }
  .tool.small { width: 20px; height: 20px; }
  .tool:hover { background: var(--hover); color: var(--label); }
  .empty { padding: 10px 14px; font-size: var(--callout); color: var(--tertiary); }
</style>
