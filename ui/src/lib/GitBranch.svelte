<script lang="ts">
  import type { PipelineStatus } from "../types";
  import PipelineStatusIcon from "./PipelineStatusIcon.svelte";

  export let name: string;
  export let pipelineStatus: PipelineStatus;
  export let pipelineUrl: string | undefined;
</script>

<div class="branch">
  <i class="fa-solid fa-code-branch" title="Branch" />
  {#if pipelineUrl}
    <a href={pipelineUrl} target="_blank">
      <PipelineStatusIcon {pipelineStatus} />
    </a>
  {:else}
    <PipelineStatusIcon {pipelineStatus} />
  {/if}
  <span class="branch-name">{name}</span>
</div>

<style>
  .branch {
    display: flex;
    align-items: center;
    padding: 0.25rem 0;
  }

  .branch-name {
    margin-left: 1rem;
  }

  .branch > :global(*:not(:first-child)) {
    margin-left: 0.5rem;
  }
</style>
