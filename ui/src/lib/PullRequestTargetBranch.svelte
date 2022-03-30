<script lang="ts">
  import GitBranch from "./GitBranch.svelte";
  import type { PullRequestTargetBranch } from "../types";
  import PullRequest from "./PullRequest.svelte";

  export let pr_target_branch: PullRequestTargetBranch;
</script>

<GitBranch
  name={pr_target_branch.branch_name}
  pipelineStatus={pr_target_branch.pipeline_status}
  pipelineUrl={pr_target_branch.pipeline_url}
/>
{#each pr_target_branch.pull_requests as pull_request}
  <PullRequest pullRequest={pull_request} />
{/each}
