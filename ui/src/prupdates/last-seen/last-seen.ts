import {
  loadStoredLastSeen,
  PullRequestUpdateLastSeen,
  storePullRequestUpdatesLastSeen,
} from './storage';
import dayjs from 'dayjs';
import { createEffect, createSignal } from 'solid-js';
import { PullRequestUpdate } from '../../types';

export const [prLastSeen, setPrLastSeen] = createSignal(loadStoredLastSeen());
createEffect(() => storePullRequestUpdatesLastSeen(prLastSeen()));

function getLastSeenUpdatedNow(prId: string): PullRequestUpdateLastSeen {
  return {
    prId,
    lastSeenTimestamp: dayjs().format(),
  };
}

export function markAllUpdatesAsLastSeenNow(updates: PullRequestUpdate[]) {
  const currentIds = updates.map((update) => update.pr_id);
  const currentLastSeen = prLastSeen();
  const notAffectedPrs = currentLastSeen.filter(
    (lastSeen) => !currentIds.includes(lastSeen.prId),
  );
  const markedUpdates: PullRequestUpdateLastSeen[] = [
    ...notAffectedPrs,
    ...currentIds.map((id) => getLastSeenUpdatedNow(id)),
  ];
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
