<script lang="ts">
  import type { ProjectMetadata } from "$lib/types/carenage";
  import { formatDate } from "$lib/utils.js";

  interface Props {
    project: ProjectMetadata;
  }

  const { project }: Props = $props();
  const formattedDate = formatDate(project.created_at);
</script>

<main>
  <div class="wrapper">
    <section class="project-metadata" aria-labelledby="project-metadata-heading">
      <h2 id="project-metadata-heading">Project: {project.project_name}</h2>
      <p>Project created on: {formattedDate}</p>
    </section>
    <section class="project-pipelines">
      <table>
        <caption>List of executed CI pipelines for the project</caption>
        <thead
          ><tr><th scope="col">Pipeline ID</th><th scope="col">Pipeline date of execution</th></tr
          ></thead
        >
        <tbody
          >{#each project.pipelines as pipeline}<tr
              ><td><a href="/pipelines/{pipeline.pipeline_id}">{pipeline.pipeline_repo_id}</a></td>
              <td>{pipeline.started_at}</td></tr
            >
          {/each}</tbody
        >
      </table>
    </section>
    <section class="project-runs">
      <table>
        <caption>List of executed CI runs for the project</caption><thead
          ><tr><th scope="col">Run ID</th><th scope="col">Run date of execution</th></tr></thead
        >
        <tbody>
          {#each project.pipelines as pipeline}
            <tr><td colspan="2">Pipeline ID #{pipeline.pipeline_repo_id}</td></tr>
            {#each pipeline.runs as run}
              <tr
                ><td><a href="/runs/{run.run_id}">{run.run_repo_id}</a></td><td>{run.started_at}</td
                ></tr
              >
            {/each}
          {/each}
        </tbody>
      </table>
    </section>
  </div>
  <section class="aggregated-metric-values" aria-labelledby="aggregated-metric-values-heading">
    <h2 id="aggregated-metric-values-heading">Project aggregated metric values</h2>
  </section>
</main>
