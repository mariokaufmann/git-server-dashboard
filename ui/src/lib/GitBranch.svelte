<script lang="ts">
  import type { PipelineStatus } from "../types";
  import PipelineStatusIcon from "./PipelineStatusIcon.svelte";

  export let name: string;
  export let pipelineStatus: PipelineStatus;
</script>

<div class="branch">
  <i class="fa-solid fa-code-branch" title="Branch"/>
  <PipelineStatusIcon {pipelineStatus} />
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
