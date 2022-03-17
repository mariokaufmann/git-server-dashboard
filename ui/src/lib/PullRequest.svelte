<script lang="ts">
  import type { PullRequest } from "../types";
  import PipelineStatusIcon from "./PipelineStatusIcon.svelte";
  import dayjs from "dayjs";

  export let pullRequest: PullRequest;
</script>

<div>
  <div class="pr">
    <i class="fa-solid fa-code-pull-request" title="Pull Request" />
    <PipelineStatusIcon pipelineStatus={pullRequest.pipeline_status} />
    <span>{pullRequest.branch_name}</span>
  </div>
  <div class="pr-metadata">
    <i
      class="fa-solid fa-comment small-icon fa-xs"
      title="Number of comments"
    />
    <span>{pullRequest.comment_count}</span>
    {#if pullRequest.approved}
      <i class="fa-solid fa-thumbs-up fa-xs" />
    {/if}
    <img
      src={pullRequest.user_profile_image}
      alt="Pull request user profile avatar"
    />
    <span class="detail">{pullRequest.user_name}</span>
    <span class="detail" title="Last updated"
      >{dayjs(pullRequest.last_activity_date).fromNow()}</span
    >
  </div>
</div>

<style>
  .pr {
    display: flex;
    align-items: center;
    padding: 0.25rem 0 0.25rem 0.5rem;
  }

  .pr > :global(*:not(:first-child)) {
    margin-left: 0.5rem;
  }

  .pr-metadata {
    display: flex;
    align-items: center;
    padding: 0.25rem 0 0.25rem 0.75rem;
  }

  .pr-metadata > :global(*:not(:first-child)) {
    margin-left: 0.5rem;
  }

  img {
    width: 22px;
    border-radius: 50%;
    border: solid 2px white;
  }

  .detail {
    display: inline-block;
    font-size: 0.8rem;
  }
</style>
