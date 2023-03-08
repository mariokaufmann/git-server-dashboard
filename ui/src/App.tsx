import {
  Component,
  createResource,
  For,
  onCleanup,
  onMount,
  Show,
} from 'solid-js';
import styles from './App.module.css';
import dayjs from 'dayjs';
import { RepositoryBranchData } from './types';
import Loader from './common/loader/Loader';
import RepositoryCard from './repositories/repository-card/RepositoryCard';
import { estimateLineCount } from './repositories/utils';
import { getDashboardData } from './repositories/fetchDashboardData';
import { getPRUpdates } from './repositories/fetchPRUpdates';
import PRUpdateCard from './prupdates/pr-update-card/PRUpdateCard';

function mapTileSizeClass(repository: RepositoryBranchData) {
  const approximateLineCount = estimateLineCount(repository);
  if (approximateLineCount > 6) {
    return styles.tileLarge;
  } else if (approximateLineCount > 2) {
    return styles.tileMedium;
  } else {
    return styles.tileSmall;
  }
}

const RELOAD_INTERVAL_MS = 2_000;

const App: Component = () => {
  const [dashboardData, { mutate, refetch }] = createResource(getDashboardData);
  const [prUpdates] = createResource(getPRUpdates);
  let timeout: number | undefined = undefined;
  const reloadData = () => {
    // refetch();
    timeout = setTimeout(reloadData, RELOAD_INTERVAL_MS);
  };
  timeout = setTimeout(reloadData, RELOAD_INTERVAL_MS);

  onMount(() =>
    document.addEventListener('visibilitychange', onVisibilityChange)
  );
  onCleanup(() => {
    clearTimeout(timeout);
    document.removeEventListener('visibilitychange', onVisibilityChange);
  });

  const onVisibilityChange = () => {
    const visibilityState = document.visibilityState;
    if (visibilityState === 'visible' && !timeout) {
      reloadData();
    } else if (visibilityState === 'hidden' && timeout) {
      clearTimeout(timeout);
      timeout = undefined;
    }
  };

  return (
    <div>
      <Show when={dashboardData()} fallback={<p>Data not loaded yet</p>} keyed>
        {(dashboardData) => (
          <>
            <div class={styles.header}>
              {dashboardData.last_updated_date && (
                <p>
                  Last updated:{' '}
                  {dayjs(dashboardData.last_updated_date).format('HH:mm:ss')}
                </p>
              )}
              {dashboardData.currently_refreshing && <Loader />}
            </div>

            <main class={styles.main}>
              <div class={styles.repositorySection}>
                <div class={styles.sectionTitle}>
                  <h2>Repositories</h2>
                </div>
                <div class={styles.repositories}>
                  <For each={dashboardData.repositories}>
                    {(repository) => (
                      <div class={mapTileSizeClass(repository)}>
                        <RepositoryCard repositoryBranchData={repository} />
                      </div>
                    )}
                  </For>
                </div>
              </div>
              <div class={styles.prUpdatesSection}>
                <div class={styles.sectionTitle}>
                  <h2>PR Updates</h2>
                  <span>
                    <p>Close all</p>
                    <i class="fa-solid fa-xmark" title="Close"></i>
                  </span>
                </div>
                <div class={styles.prUpdates}>
                  <For each={prUpdates()}>
                    {(prUpdate) => (
                      <PRUpdateCard prUpdate={prUpdate}></PRUpdateCard>
                    )}
                  </For>
                </div>
              </div>
            </main>
          </>
        )}
      </Show>
    </div>
  );
};

export default App;
