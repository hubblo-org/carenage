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
  <section class="project-metadata" aria-labelledby="project-metadata-heading">
    <h2 id="project-metadata-heading">Project: {project.project_name}</h2>
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
            <td>{run.started_at}</td></tr
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
  </section>
  <section class="aggregated-metric-values" aria-labelledby="aggregated-metric-values-heading">
    <h2 id="aggregated-metric-values-heading">Project's aggregated metric values</h2>
  </section>
</main>
