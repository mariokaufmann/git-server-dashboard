<script lang="ts">
  import type { PipelineStatus } from "../types";

  const mapToIcon = (status: PipelineStatus): string => {
    switch (status) {
      case "Successful":
        return "circle-check";
      case "Failed":
        return "circle-xmark";
      case "Running":
        return "circle-stop";
      case "Canceled":
        return "circle-trash";
      case "Queued":
        return "circle-pause";
      case "None":
        return "circle-question";
    }
  };

  const mapToColorClass = (status: PipelineStatus): string => {
    switch (status) {
      case "Successful":
        return "icon-successful";
      case "Failed":
        return "icon-failed";
      case "Running":
        return "icon-running";
      case "Canceled":
        return "icon-failed";
      case "Queued":
        return "icon-default";
      case "None":
        return "icon-default";
    }
  };

  const mapToTitle = (status: PipelineStatus): string => {
    switch (status) {
      case "Successful":
        return "Pipeline successful";
      case "Failed":
        return "Pipeline failed";
      case "Running":
        return "Pipeline running";
      case "Canceled":
        return "Pipeline canceled";
      case "Queued":
        return "Pipeline queued";
      case "None":
        return "No pipeline found";
    }
  };

  export let pipelineStatus: PipelineStatus;
</script>

<i
  class="fa-solid fa-{mapToIcon(pipelineStatus)} {mapToColorClass(
    pipelineStatus
  )}"
  title={mapToTitle(pipelineStatus)}
/>

<style>
  i {
    padding: 2px;
    border-radius: 50%;
    background-color: white;
    box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.2);
  }

  .icon-successful {
    color: var(--color-green);
  }

  .icon-failed {
    color: var(--color-fiery-rose);
  }

  .icon-running {
    color: var(--color-blue-jeans);
  }

  .icon-default {
    color: var(--color-text);
  }
</style>
