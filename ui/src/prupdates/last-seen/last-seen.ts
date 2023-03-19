import { PullRequestUpdate } from '../../types';
import {
  storePullRequestUpdateLastSeen,
  storePullRequestUpdatesLastSeen,
} from './storage';
import dayjs from 'dayjs';

export function markAllUpdatesAsLastSeenNow(
  updates: PullRequestUpdate[] | undefined
) {
  if (updates) {
    storePullRequestUpdatesLastSeen(
      Object.fromEntries(
        updates.map((update) => [update.pr_id, dayjs().format()])
      )
    );
  }
}

export function markUpdateAsLastSeenNow(update: PullRequestUpdate) {
  storePullRequestUpdateLastSeen(update.pr_id, dayjs().format());
}
