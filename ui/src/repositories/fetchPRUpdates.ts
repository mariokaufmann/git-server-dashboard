import { PullRequestEvent } from '../types';

export async function getPRUpdates(): Promise<PullRequestEvent[]> {
  const res = await fetch('/api/pr-updates');
  const text = await res.text();

  if (res.ok) {
    return JSON.parse(text);
  } else {
    throw new Error(text);
  }
}
