import { Component } from 'solid-js';
import styles from './GitBranch.module.css';
import { PipelineStatus } from '../../types';
import PipelineStatusIcon from '../pipeline-status-icon/PipelineStatusIcon';
const GitBranch: Component<{
  name: string;
  pipelineStatus: PipelineStatus;
  pipelineUrl: string | undefined;
}> = (props) => {
  return (
    <div class={styles.branch}>
      <i class="fa-solid fa-code-branch" title="Branch" />
      {props.pipelineUrl ? (
        <a href={props.pipelineUrl} target="_blank">
          <PipelineStatusIcon pipelineStatus={props.pipelineStatus} />
        </a>
      ) : (
        <PipelineStatusIcon pipelineStatus={props.pipelineStatus} />
      )}
      <span class={styles.branchName}>{props.name}</span>
    </div>
  );
};
export default GitBranch;
