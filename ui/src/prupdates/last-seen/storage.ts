import dayjs from 'dayjs';

interface LastSeenStorageItem {
  lastSeen: { [key: string]: string };
}

export interface PullRequestUpdateLastSeen {
  prId: string;
  lastSeenTimestamp: string;
}

const STORAGE_KEY = 'prUpdateLastSeen';

export function getPullRequestUpdatesLastSeen(): PullRequestUpdateLastSeen[] {
  const storedLastSeen = localStorage.getItem(STORAGE_KEY);
  if (!storedLastSeen) {
    return [];
  }
  const parsedStoredLastSeen = JSON.parse(
    storedLastSeen
  ) as LastSeenStorageItem;
  return Object.entries(parsedStoredLastSeen.lastSeen).map(
    ([prId, lastSeenTimestamp]) => ({
      prId,
      lastSeenTimestamp,
    })
  );
}

// we clean up old entries in the last seen list after 1 month
function lastSeenWithinThreshold(timestamp: string): boolean {
  const parsedTimestamp = dayjs(timestamp);
  const ageThreshold = dayjs().subtract(1, 'month');
  return parsedTimestamp.isAfter(ageThreshold);
}

export function storePullRequestUpdatesLastSeen(lastSeen: {
  [key: string]: string;
}) {
  const filteredLastSeen = Object.entries(lastSeen).filter(([_, timestamp]) =>
    lastSeenWithinThreshold(timestamp)
  );

  const lastSeenItem: LastSeenStorageItem = {
    lastSeen: Object.fromEntries(filteredLastSeen),
  };
  const serializedLastSeen = JSON.stringify(lastSeenItem);
  localStorage.setItem(STORAGE_KEY, serializedLastSeen);
}

export function storePullRequestUpdateLastSeen(
  prId: string,
  timestamp: string
) {
  const storedLastSeen = localStorage.getItem(STORAGE_KEY);
  let newMappings;
  if (storedLastSeen) {
    const parsedStoredLastSeen = JSON.parse(
      storedLastSeen
    ) as LastSeenStorageItem;
    newMappings = {
      ...parsedStoredLastSeen.lastSeen,
      [prId]: timestamp,
    };
  } else {
    newMappings = { [prId]: timestamp };
  }
  storePullRequestUpdatesLastSeen(newMappings);
}
