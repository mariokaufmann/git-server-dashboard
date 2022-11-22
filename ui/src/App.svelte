<script lang="ts">
  import { getDashboardData } from "./api/dashboardData";
  import RepositoryCard from "./lib/RepositoryCard.svelte";
  import dayjs from "dayjs";
  import type { DashboardData, RepositoryBranchData } from "./types";
  import { onDestroy } from "svelte";
  import Loader from "./lib/Loader.svelte";

  let dashboardData: DashboardData;
  let timeout = null;

  function reloadData() {
    getDashboardData().then((data) => {
      dashboardData = {
        last_updated_date: data.last_updated_date,
        repositories: data.repositories.sort(
          (rep1, rep2) => estimateLineCount(rep2) - estimateLineCount(rep1)
        ),
        currently_refreshing: data.currently_refreshing,
      };
    });
    timeout = setTimeout(reloadData, 2_000);
  }

  function onVisibilityChange() {
    const visibilityState = document.visibilityState;
    if (visibilityState === "visible" && !timeout) {
      reloadData();
    } else if (visibilityState === "hidden" && timeout) {
      clearTimeout(timeout);
      timeout = null;
    }
  }

  function estimateLineCount(repository: RepositoryBranchData) {
    return (
      repository.standalone_branches.length +
      repository.pull_request_target_branches.length +
      repository.pull_request_target_branches.reduce(
        (previous, current) => previous + 2 * current.pull_requests.length,
        0
      )
    );
  }

  function mapTileSizeClass(repository: RepositoryBranchData) {
    const approximateLineCount = estimateLineCount(repository);
    if (approximateLineCount > 6) {
      return "tile-large";
    } else if (approximateLineCount > 2) {
      return "tile-medium";
    } else {
      return "tile-small";
    }
  }

  reloadData();
  document.addEventListener("visibilitychange", onVisibilityChange);

  onDestroy(() => {
    if (timeout) {
      clearTimeout(timeout);
    }
    document.removeEventListener("visibilitychange", onVisibilityChange);
  });
</script>

{#if dashboardData}
  <div class="header">
    {#if dashboardData.last_updated_date}
      <p>
        Last updated: {dayjs(dashboardData.last_updated_date).format(
          "HH:mm:ss"
        )}
      </p>
    {:else}
      <p>Data not loaded yet</p>
    {/if}
    {#if dashboardData.currently_refreshing}
      <Loader />
    {/if}
  </div>
  <main>
    {#each dashboardData.repositories as repository}
      <div class={mapTileSizeClass(repository)}>
        <RepositoryCard repositoryBranchData={repository} />
      </div>
    {/each}
  </main>
{/if}

<style>
  main {
    margin: 0 1rem;
    display: grid;
    grid-gap: 1rem;
    color: var(--color-text);
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    grid-auto-flow: dense;
  }

  .tile-large {
    grid-row: span 3;
  }

  .tile-medium {
    grid-row: span 2;
  }

  .tile-small {
    grid-row: span 1;
  }

  .header {
    display: flex;
    padding: 0.5rem 1rem;
    justify-content: flex-end;
    width: 100%;
  }

  .header p {
    margin: 0 0.5rem;
    font-size: 0.8em;
    height: 1rem;
  }
</style>
