import { Component, For } from 'solid-js';
import styles from './RepositoryCard.module.css';
import { RepositoryBranchData } from '../../types';
import PullRequestTargetBranch from '../pull-request-target-branch/PullRequestTargetBranch';
import GitBranch from '../git-branch/GitBranch';

const RepositoryCard: Component<{
  repositoryBranchData: RepositoryBranchData;
}> = (props) => {
  return (
    <div class={styles.card}>
      <a href={props.repositoryBranchData.repository_url} target="_blank">
        <h2>{props.repositoryBranchData.repository_name}</h2>
      </a>
      <div class={styles.scrollable}>
        <For each={props.repositoryBranchData.pull_request_target_branches}>
          {(pr_target_branch) => (
            <PullRequestTargetBranch pr_target_branch={pr_target_branch} />
          )}
        </For>
        <For each={props.repositoryBranchData.standalone_branches}>
          {(standalone_branch) => (
            <GitBranch
              pipelineStatus={standalone_branch.pipeline_status}
              pipelineUrl={standalone_branch.pipeline_url}
              name={standalone_branch.branch_name}
            />
          )}
        </For>
      </div>
    </div>
  );
};
export default RepositoryCard;
