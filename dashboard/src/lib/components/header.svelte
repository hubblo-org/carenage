<script lang="ts">
  import type { ProjectMetadata } from "$lib/types/carenage";
  import { page } from "$app/stores";

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

    <a href={project.project_url}> <h2>{project.project_name}</h2></a>
  </div>
  <div class="project-metadata">
    <p>Number of pipelines executed: {numberOfPipelines}</p>
    <p>Number of job runs executed: {numberOfRuns}</p>
  </div>
  <nav>
    <a href="/projects/{project.project_id}">Project summary and metrics</a>
    {#if $page.data?.run}<a href="/pipelines/{$page.data.run.pipeline_id}"
        >Pipeline summary and metrics
      </a>
    {/if}
  </nav>
</header>

<style>
  header {
    display: flex;
  }
  nav {
    display: flex;
    width: 70%;
    justify-content: space-around;
  }
  nav a {
    text-decoration: none;
    border: 2px solid black;
    padding: 12px;
    display: flex;
    align-items: center;
  }
  nav a:hover {
    background-color: var(--blue-100);
    color: black;
  }
  .project-metadata {
    margin-left: 24px;
  }
</style>
