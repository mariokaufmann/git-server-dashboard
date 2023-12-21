import { Component } from 'solid-js';
import styles from './PullRequest.module.css';
import PipelineStatusIcon from '../pipeline-status-icon/PipelineStatusIcon';
import { PullRequest } from '../../types';
import dayjs from 'dayjs';

const PullRequestComponent: Component<{ pullRequest: PullRequest }> = (
  props,
) => {
  return (
    <div>
      <div class={styles.pr}>
        <a href={props.pullRequest.link_url} target="_blank">
          <i class="fa-solid fa-code-pull-request" title="Pull Request" />
        </a>
        {props.pullRequest.pipeline_url ? (
          <a href={props.pullRequest.pipeline_url} target="_blank">
            <PipelineStatusIcon
              pipelineStatus={props.pullRequest.pipeline_status}
            />
          </a>
        ) : (
          <PipelineStatusIcon
            pipelineStatus={props.pullRequest.pipeline_status}
          />
        )}
        <span class={styles.branchName}>
          <a href={props.pullRequest.link_url} target="_blank">
            {props.pullRequest.branch_name}
          </a>
        </span>
      </div>
      <div class={styles.prMetadata}>
        <i
          class="fa-solid fa-comment small-icon fa-xs"
          title="Number of comments"
        />
        <span>{props.pullRequest.comment_count}</span>
        {props.pullRequest.approved && (
          <i class="fa-solid fa-thumbs-up fa-xs" />
        )}
        <img
          src={props.pullRequest.user_profile_image}
          alt="Pull request user profile avatar"
        />
        <span class={styles.detail}>{props.pullRequest.user_name}</span>
        <span class={styles.detail} title="Last updated">
          {dayjs(props.pullRequest.last_activity_date).fromNow()}
        </span>
      </div>
    </div>
  );
};
export default PullRequestComponent;
