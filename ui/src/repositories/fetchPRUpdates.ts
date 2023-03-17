import { GetPullRequestUpdatesPayload, PullRequestUpdate } from '../types';

export async function getPRUpdates(): Promise<PullRequestUpdate[]> {
  const payload: GetPullRequestUpdatesPayload = {
    pull_requests_last_seen: [],
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
