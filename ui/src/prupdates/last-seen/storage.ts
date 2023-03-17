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

export function storePullRequestUpdatesLastSeen(lastSeen: {
  [key: string]: string;
}) {
  const lastSeenItem: LastSeenStorageItem = {
    lastSeen,
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
