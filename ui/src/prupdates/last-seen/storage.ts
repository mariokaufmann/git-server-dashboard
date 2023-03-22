import dayjs from 'dayjs';

const CurrentVersion = 1;

interface StoredPullRequestUpdateLastSeen {
  prId: string;
  lastSeenTimestamp: string;
}

interface LastSeenStorageItem {
  version: number;

  pullRequests: StoredPullRequestUpdateLastSeen[];
}

export interface PullRequestUpdateLastSeen {
  prId: string;
  lastSeenTimestamp: string;
}

const STORAGE_KEY = 'prUpdateLastSeen';

export function loadStoredLastSeen(): PullRequestUpdateLastSeen[] {
  const storedLastSeen = localStorage.getItem(STORAGE_KEY);
  if (!storedLastSeen) {
    return [];
  }
  const parsedLastSeen: LastSeenStorageItem = JSON.parse(storedLastSeen);
  if (!parsedLastSeen.version || parsedLastSeen.version !== CurrentVersion) {
    console.log('Found old version of last seen, clearing.');
    localStorage.removeItem(STORAGE_KEY);
    return [];
  }
  return cleanupLastSeen(parsedLastSeen.pullRequests);
}

export function storePullRequestUpdatesLastSeen(
  lastSeen: PullRequestUpdateLastSeen[]
) {
  const storedLastSeen: LastSeenStorageItem = {
    version: CurrentVersion,
    pullRequests: lastSeen,
  };
  localStorage.setItem(STORAGE_KEY, JSON.stringify(storedLastSeen));
}

// we clean up old entries in the last seen list after 1 month
function cleanupLastSeen(
  lastSeen: PullRequestUpdateLastSeen[]
): PullRequestUpdateLastSeen[] {
  const filteredLastSeen = lastSeen.filter((pullRequest) =>
    lastSeenWithinThreshold(pullRequest.lastSeenTimestamp)
  );
  storePullRequestUpdatesLastSeen(filteredLastSeen);
  return filteredLastSeen;
}

function lastSeenWithinThreshold(timestamp: string): boolean {
  const parsedTimestamp = dayjs(timestamp);
  const ageThreshold = dayjs().subtract(1, 'month');
  return parsedTimestamp.isAfter(ageThreshold);
}
