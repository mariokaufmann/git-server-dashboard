import { Component, ParentComponent } from 'solid-js';
import styles from './Card.module.css';
const Card: ParentComponent = (props) => {
  return <div class={styles.Card} data-testid="card">{props.children}</div>;
};
export default Card;
