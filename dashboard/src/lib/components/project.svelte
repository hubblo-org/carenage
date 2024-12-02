<script lang="ts">
  import type { ProjectRecord } from "$lib/types/carenage";
  import { formatDate } from "$lib/utils.js";

  interface Props {
    project: ProjectRecord;
  }

  const { project }: Props = $props();
  const formattedDate = formatDate(project.created_at);
</script>

<main>
  <h2>Project: {project.project_name}</h2>
  <p>Project created on: {formattedDate}</p>
  <table>
    <caption>List of executed CI pipelines for the project</caption>
    <thead
      ><tr><th scope="col">Pipeline ID</th><th scope="col">Pipeline date of execution</th></tr
      ></thead
    >
    <tbody
      >{#each project.runs as run}<tr
          ><td><a href="/pipelines/{run.pipeline_id}">{run.pipeline_id}</a></td>
          <td>{run.created_at}</td></tr
        >
      {/each}</tbody
    >
  </table>
  <table>
    <caption>List of executed CI runs for the project</caption><thead
      ><tr><th scope="col">Run ID</th><th scope="col">Run date of execution</th></tr></thead
    >
    <tbody>
      {#each project.runs as run}
        <tr><td><a href="/runs/{run.run_id}">{run.run_id}</a></td><td>{run.started_at}</td></tr>
      {/each}
    </tbody>
  </table>
</main>
