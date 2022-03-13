<script lang="ts">
  import type { PipelineStatus } from "../types";
  import PipelineStatusIcon from "./PipelineStatusIcon.svelte";

  export let name: string;
  export let pipelineStatus: PipelineStatus;
</script>

<div class="branch">
  <i class="fa-solid fa-code-branch" />
  <PipelineStatusIcon {pipelineStatus} />
  <span>{name}</span>
</div>

<style>
  .branch {
    display: flex;
    align-items: center;
  }

  .branch i {
    margin-left: 0.5rem;
  }
</style>
