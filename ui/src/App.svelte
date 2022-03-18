<script lang="ts">
  import { getDashboardData } from "./api/dashboardData";
  import RepositoryCard from "./lib/RepositoryCard.svelte";

  let dashboardDataPromise = getDashboardData();
</script>

<main>
  {#await dashboardDataPromise}
    <p>Loading...</p>
  {:then dashboardData}
    {#each dashboardData.repositories as repository}
      <RepositoryCard repositoryBranchData={repository} />
    {/each}
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</main>

<style>
  main {
    margin: 50px 50px 0 50px;
    display: flex;
    flex-wrap: wrap;
    column-gap: 1rem;
    row-gap: 1rem;
  }
</style>
