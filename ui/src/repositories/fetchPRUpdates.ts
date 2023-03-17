import { GetPullRequestUpdatesPayload, PullRequestUpdate } from '../types';
import { PullRequestUpdateLastSeen } from '../prupdates/last-seen/storage';

export async function getPRUpdates(
  last_seen: PullRequestUpdateLastSeen[]
): Promise<PullRequestUpdate[]> {
  const payload: GetPullRequestUpdatesPayload = {
    pull_requests_last_seen: last_seen.map((item) => ({
      pr_id: item.prId,
      last_seen_timestamp: item.lastSeenTimestamp,
    })),
  };
  const res = await fetch('/api/pr-updates', {
    method: 'POST',
    body: JSON.stringify(payload),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const text = await res.text();

  if (res.ok) {
    return JSON.parse(text);
  } else {
    throw new Error(text);
  }
}
