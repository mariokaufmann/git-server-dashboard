import { Component, For } from 'solid-js';
import GitBranch from '../git-branch/GitBranch';
import { PullRequestTargetBranch } from '../../types';
import PullRequest from '../pull-request/PullRequest';

const PullRequestTargetBranchComponent: Component<{
  pr_target_branch: PullRequestTargetBranch;
}> = (props) => {
  return (
    <>
      <GitBranch
        name={props.pr_target_branch.branch_name}
        pipelineStatus={props.pr_target_branch.pipeline_status}
        pipelineUrl={props.pr_target_branch.pipeline_url}
      />
      <For each={props.pr_target_branch.pull_requests}>
        {(pull_request) => <PullRequest pullRequest={pull_request} />}
      </For>
    </>
  );
};
export default PullRequestTargetBranchComponent;
