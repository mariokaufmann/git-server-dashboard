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
      case "None":
        return "icon-none";
    }
  };

  export let pipelineStatus: PipelineStatus;
</script>

<i
  class="fa-solid fa-{mapToIcon(pipelineStatus)} {mapToColorClass(
    pipelineStatus
  )}"
/>

<style>
  i {
    padding: 2px;
    border-radius: 50%;
    background-color: white;
    box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.2);
  }

  .icon-successful {
    color: var(--color-screaming-green);
    background-color: black;
  }

  .icon-failed {
    color: var(--color-fiery-rose);
  }

  .icon-running {
    color: var(--color-blue-jeans);
  }

  .icon-none {
    color: var(--color-text);
  }
</style>
