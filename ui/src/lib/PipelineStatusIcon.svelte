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

  export let pipelineStatus: PipelineStatus;
</script>

<i class="fa-solid fa-{mapToIcon(pipelineStatus)}" />
