<script lang="ts">
  import type { RepositoryBranchData } from "../types";
  import PullRequestTargetBranch from "./PullRequestTargetBranch.svelte";
  import GitBranch from "./GitBranch.svelte";

  export let repositoryBranchData: RepositoryBranchData;
</script>

<div class="card">
  <a href={repositoryBranchData.repository_url} target="_blank">
    <h2>{repositoryBranchData.repository_name}</h2>
  </a>
  <div class="scrollable">
    {#each repositoryBranchData.pull_request_target_branches as pr_target_branch}
      <PullRequestTargetBranch {pr_target_branch} />
    {/each}
    {#each repositoryBranchData.standalone_branches as standalone_branch}
      <GitBranch
        pipelineStatus={standalone_branch.pipeline_status}
        pipelineUrl={standalone_branch.pipeline_url}
        name={standalone_branch.branch_name}
      />
    {/each}
  </div>
</div>

<style>
  .card {
    width: 100%;
    height: 100%;
    background: radial-gradient(50% 50% at 50% 50%, #e8eaef 0%, #e5e5fc 100%);
    border-radius: 5px;
    padding: 1rem;
    box-shadow: 5px 5px 5px rgba(0, 0, 0, 0.2);
  }

  .scrollable {
    overflow-y: auto;
  }

  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 22px;
  }
</style>
