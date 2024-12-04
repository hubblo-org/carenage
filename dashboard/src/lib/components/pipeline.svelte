<script lang="ts">
  import type { CiPipeline } from "$lib/types/carenage";
  import { formatTime, formatDuration } from "$lib/utils";

  interface Props {
    pipeline: CiPipeline;
  }

  const { pipeline }: Props = $props();
  const pipelineDuration = formatDuration(pipeline.duration);
  const pipelineStartTime = formatTime(pipeline.started_at);
  const pipelineEndTime = formatTime(pipeline.finished_at);
</script>

<main>
  <div class="wrapper">
    <section aria-labelledby="pipeline-metadata-heading">
      <h2 id="pipeline-metadata-heading">Pipeline #{pipeline.pipeline_repo_id} metadata</h2>
      <a href={pipeline.project_repo_url}>{pipeline.project_name}</a>
      <a href={pipeline.pipeline_repo_url}>Pipeline #{pipeline.pipeline_repo_id}</a>
      <p>Pipeline duration: {pipelineDuration}</p>
      <p>Pipeline started at: {pipelineStartTime}</p>
      <p>Pipeline finished at: {pipelineEndTime}</p>
    </section>
    <section aria-labelledby="pipeline-runs-heading">
      <h2 id="pipeline-runs-heading">Executed runs for pipeline #{pipeline.pipeline_repo_id}</h2>
      <table>
        <caption>List of executed CI runs for the pipeline</caption><thead
          ><tr><th scope="col">Run ID</th><th scope="col">Run start time of execution</th></tr
          ></thead
        >
        <tbody
          >{#each pipeline.runs as run}<tr
              ><td><a href="/runs/{run.run_id}">Run #{run.run_repo_id}</a></td><td
                >{formatTime(run.started_at)}</td
              ></tr
            >{/each}</tbody
        >
      </table>
    </section>
  </div>
  <section class="aggregated-metric-values" aria-labelledby="aggregated-metric-values-heading">
    <h2 id="aggregated-metric-values-heading">Pipeline aggregated metric values</h2>
  </section>
</main>
