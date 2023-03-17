import { Component, For } from 'solid-js';
import styles from './PRUpdateCard.module.css';
import Card from '../../common/card/Card';
import dayjs from 'dayjs';
import { PullRequestUpdate, PullRequestUpdateType } from '../../types';

// TODO use fa-code-commit for new commits
const updateIconMaps: { [key in PullRequestUpdateType]: string } = {
  PRApproved: 'fa-thumbs-up',
  PRCommentAdded: 'fa-comment',
  PRMerged: 'fa-code-merge',
  PROpened: 'fa-code-branch',
  Aggregated: 'fa-layer-group',
};

const PRUpdateCard: Component<{
  prUpdate: PullRequestUpdate;
  markAsLastSeenNow: () => void;
}> = (props) => {
  return (
    <Card>
      <div class={styles.content}>
        <div class={styles.textContent}>
          <div class={styles.title}>
            <i
              classList={{
                'fa-solid': true,
                'fa-l': true,
                [updateIconMaps[props.prUpdate.update_type]]: true,
              }}
              title="Close"
            ></i>
            <h3>{props.prUpdate.repository}</h3>
          </div>
          <p>{props.prUpdate.title}</p>
          <div class={styles.details}>
            <div class={styles.detailsText}>
              {props.prUpdate.details.length > 0 && (
                <ul>
                  <For each={props.prUpdate.details}>
                    {(detail) => <li>{detail}</li>}
                  </For>
                </ul>
              )}
            </div>
          </div>
          <div class={styles.footer}>
            <span class={styles.detail}>{props.prUpdate.author}</span>
            <span class={styles.detail} title="Last updated">
              {dayjs(props.prUpdate.timestamp).fromNow()}
            </span>
          </div>
        </div>
        <div class={styles.close}>
          <i
            class="fa-solid fa-xmark"
            title="Close"
            onClick={() => props.markAsLastSeenNow()}
          ></i>
        </div>
      </div>
    </Card>
  );
};
export default PRUpdateCard;
