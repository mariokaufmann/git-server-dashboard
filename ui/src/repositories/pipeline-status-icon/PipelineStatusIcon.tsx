import { Component } from 'solid-js';
import styles from './PipelineStatusIcon.module.css';
import { PipelineStatus } from '../../types';

const mapToIcon = (status: PipelineStatus): string => {
  switch (status) {
    case 'Successful':
      return 'circle-check';
    case 'Failed':
      return 'circle-xmark';
    case 'Running':
      return 'circle-stop';
    case 'Canceled':
      return 'circle-minus';
    case 'Queued':
      return 'circle-pause';
    case 'None':
      return 'circle-question';
  }
};

const mapToColorClass = (status: PipelineStatus): string => {
  switch (status) {
    case 'Successful':
      return styles.iconSuccessful;
    case 'Failed':
      return styles.iconFailed;
    case 'Running':
      return styles.iconRunning;
    case 'Canceled':
      return styles.iconFailed;
    case 'Queued':
      return styles.iconDefault;
    case 'None':
      return styles.iconDefault;
  }
};

const mapToTitle = (status: PipelineStatus): string => {
  switch (status) {
    case 'Successful':
      return 'Pipeline successful';
    case 'Failed':
      return 'Pipeline failed';
    case 'Running':
      return 'Pipeline running';
    case 'Canceled':
      return 'Pipeline canceled';
    case 'Queued':
      return 'Pipeline queued';
    case 'None':
      return 'No pipeline found';
  }
};

const PipelineStatusIcon: Component<{ pipelineStatus: PipelineStatus }> = (
  props
) => {
  return (
    <i
      classList={{
        [styles.icon]: true,
        'fa-solid': true,
        [`fa-${mapToIcon(props.pipelineStatus)}`]: true,
        [mapToColorClass(props.pipelineStatus)]: true,
      }}
      class="fa-solid fa-{mapToIcon(pipelineStatus)} {mapToColorClass(
    pipelineStatus
  )}"
      title={mapToTitle(props.pipelineStatus)}
    />
  );
};
export default PipelineStatusIcon;
