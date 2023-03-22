import {
  loadStoredLastSeen,
  PullRequestUpdateLastSeen,
  storePullRequestUpdatesLastSeen,
} from './storage';
import dayjs from 'dayjs';
import { createEffect, createSignal } from 'solid-js';

export const [prLastSeen, setPrLastSeen] = createSignal(loadStoredLastSeen());
createEffect(() => storePullRequestUpdatesLastSeen(prLastSeen()));

function getLastSeenUpdatedNow(prId: string): PullRequestUpdateLastSeen {
  return {
    prId,
    lastSeenTimestamp: dayjs().format(),
  };
}

export function markAllUpdatesAsLastSeenNow() {
  const updates = prLastSeen();
  const markedUpdates: PullRequestUpdateLastSeen[] = updates.map(
    (currentUpdate) => getLastSeenUpdatedNow(currentUpdate.prId)
  );
  setPrLastSeen(markedUpdates);
}

export function markUpdateAsLastSeenNow(prId: string) {
  const updates = prLastSeen();
  const markedUpdates: PullRequestUpdateLastSeen[] = [
    ...updates.filter((currentUpdate) => currentUpdate.prId !== prId),
    getLastSeenUpdatedNow(prId),
  ];
  setPrLastSeen(markedUpdates);
}
