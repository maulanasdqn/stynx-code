<script>
  import { mode, thinking } from "../../lib/stores.js";
  import { applyMode, applyThinking } from "../../lib/session-actions.js";

  const MODES = [
    { value: "normal", label: "Normal" },
    { value: "auto", label: "Auto" },
    { value: "plan", label: "Plan" },
    { value: "bypass", label: "Bypass" },
  ];
</script>

<section>
  <div class="sb-header">Permission</div>
  <label class="sb-row">
    Mode
    <select value={$mode} on:change={(event) => applyMode(event.target.value)}>
      {#each MODES as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  </label>
  <label class="sb-row">
    <input
      type="checkbox"
      checked={$thinking}
      on:change={(event) => applyThinking(event.target.checked)}
    />
    Thinking
  </label>
</section>

<style>
  input[type="checkbox"] {
    accent-color: var(--accent);
    margin: 0;
  }
</style>
