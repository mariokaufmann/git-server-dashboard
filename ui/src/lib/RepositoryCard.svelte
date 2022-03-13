<script lang="ts">
  import type { RepositoryBranchData } from "../types";
  import GitBranch from "./GitBranch.svelte";

  export let repositoryBranchData: RepositoryBranchData;
</script>

<div class="card">
  <h2>{repositoryBranchData.repository_name}</h2>
  <div class="scrollable">
    {#each repositoryBranchData.pull_request_target_branches as pr_target_branch}
      <GitBranch
        name={pr_target_branch.branch_name}
        pipelineStatus={pr_target_branch.pipeline_status}
      />
      {#each pr_target_branch.pull_requests as pull_request}
        <p>{pull_request.branch_name}</p>
        <img
          src={pull_request.user_profile_image}
          alt="Pull request user profile avatar"
        />
        <i class="fa-solid fa-code-branch" />
      {/each}
    {/each}
  </div>
</div>

<style>
  .card {
    background: radial-gradient(50% 50% at 50% 50%, #e8eaef 0%, #b5bdd0 100%);
    border-radius: 5px;
    padding: 1rem;
  }

  .scrollable {
    height: 10rem;
    overflow-y: auto;
  }

  h2 {
    margin: 0 0 1rem 0;
  }

  img {
    width: 30px;
    border-radius: 50%;
  }
</style>
