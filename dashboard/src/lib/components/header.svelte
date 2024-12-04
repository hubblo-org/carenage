<script lang="ts">
  import type { ProjectMetadata } from "$lib/types/carenage";

  interface Props {
    project: ProjectMetadata;
  }

  const { project }: Props = $props();
  const numberOfPipelines = $derived(project.pipelines.length);
  const numberOfRuns = $derived(
    project.pipelines.map((pipeline) => pipeline.runs.length).reduce((acc, val) => acc + val, 0)
  );
</script>

<header>
  <div>
    <h1>Carenage</h1>

    <a href={`/projects/${project.project_id}`}> <h2>{project.project_name}</h2></a>
  </div>
  <div>
    <p>Number of pipelines executed: {numberOfPipelines}</p>
    <p>Number of job runs executed: {numberOfRuns}</p>
  </div>
</header>

<style>
  header {
    display: flex;
  }
</style>
