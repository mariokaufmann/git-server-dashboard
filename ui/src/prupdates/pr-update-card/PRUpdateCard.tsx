import { Component, For } from 'solid-js';
import styles from './PrUpdateCard.module.css';
import Card from '../../common/card/Card';
import dayjs from 'dayjs';
import { PullRequestUpdate } from '../../types';

const PRUpdateCard: Component<{ prUpdate: PullRequestUpdate }> = (props) => {
  return (
    <Card>
      <div class={styles.content}>
        <div class={styles.textContent}>
          <h3>{props.prUpdate.repository}</h3>
          <p>{props.prUpdate.title}</p>
          {props.prUpdate.details.length === 1 && (
            <p>props.prUpdate.details[0]</p>
          )}
          {props.prUpdate.details.length > 0 && (
            <ul>
              <For each={props.prUpdate.details}>
                {(detail) => <li>{detail}</li>}
              </For>
            </ul>
          )}
          <div class={styles.footer}>
            <span class={styles.detail}>{props.prUpdate.author}</span>
            <span class={styles.detail} title="Last updated">
              {dayjs(props.prUpdate.timestamp).fromNow()}
            </span>
          </div>
        </div>
        <div class={styles.close}>
          <i class="fa-solid fa-xmark" title="Close"></i>
        </div>
      </div>
    </Card>
  );
};
export default PRUpdateCard;
