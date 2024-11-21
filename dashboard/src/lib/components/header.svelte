<script lang="ts">
  import type { CiRun } from "../../types/carenage";

  interface Props {
    run: CiRun;
  }

  const { run }: Props = $props();
</script>

<header>
  <div>
    <h1>Carenage</h1>
    <h2>{run.project_name}</h2>
    <a href={`https://gitlab.com/${run.project_name}`}> Link to your project's repository </a>
  </div>
  <div>
    <p>Number of pipelines executed</p>
    <p>Number of job runs executed</p>
  </div>
</header>

<style>
  header {
    display: flex;
  }
</style>
