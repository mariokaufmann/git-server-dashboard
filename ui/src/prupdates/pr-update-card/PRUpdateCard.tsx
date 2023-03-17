import { Component } from 'solid-js';
import styles from './PrUpdateCard.module.css';
import Card from '../../common/card/Card';
import { PullRequestEvent } from '../../types';
import dayjs from 'dayjs';

const PRUpdateCard: Component<{ prUpdate: PullRequestEvent }> = (props) => {
  return (
    <Card>
      <div class={styles.content}>
        <div class={styles.textContent}>
          <h3>{props.prUpdate.repository}</h3>
          <p>{props.prUpdate.title}</p>
          {props.prUpdate.text.length > 0 && <p>{props.prUpdate.text}</p>}
          <span class={styles.detail}>{props.prUpdate.author}</span>
          <span class={styles.detail} title="Last updated">
            {dayjs(props.prUpdate.timestamp).fromNow()}
          </span>
        </div>
        <div class={styles.close}>
          <i class="fa-solid fa-xmark" title="Close"></i>
        </div>
      </div>
    </Card>
  );
};
export default PRUpdateCard;
