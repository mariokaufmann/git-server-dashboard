<script lang="ts">
  import { getDashboardData } from "./api/dashboardData";
  import RepositoryCard from "./lib/RepositoryCard.svelte";
  import dayjs from "dayjs";
  import type { DashboardData } from "./types";
  import { onDestroy } from "svelte";

  let dashboardData: DashboardData;
  let timeout = null;

  function reloadData() {
    getDashboardData().then((data) => (dashboardData = data));
    timeout = setTimeout(reloadData, 10_000);
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
  <main>
    {#each dashboardData.repositories as repository}
      <RepositoryCard repositoryBranchData={repository} />
    {/each}
  </main>
  <div class="footer">
    {#if dashboardData.last_updated_date}
      <p>
        Last updated: {dayjs(dashboardData.last_updated_date).format(
          "HH:mm:ss"
        )}
      </p>
    {:else}
      <p>Data not loaded yet</p>
    {/if}
  </div>
{/if}

<style>
  main {
    margin: 50px 50px 0 50px;
    display: flex;
    flex-wrap: wrap;
    column-gap: 1rem;
    row-gap: 1rem;
    color: var(--color-text);
  }

  .footer {
    position: absolute;
    bottom: 0;
    display: flex;
    padding: 1rem;
    justify-content: flex-end;
    width: 100%;
  }

  .footer p {
    margin: 0;
    font-size: 0.8em;
  }
</style>
