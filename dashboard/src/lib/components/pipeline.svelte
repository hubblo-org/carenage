<script lang="ts">
  import type { CiPipeline } from "$lib/types/carenage";
  import { JobStatus } from "$lib/types/enums";
  import { formatTime, formatDuration } from "$lib/utils";

  interface Props {
    pipeline: CiPipeline;
  }

  const { pipeline }: Props = $props();
  const pipelineDuration = formatDuration(pipeline.duration);
  const pipelineStartTime = formatTime(pipeline.started_at);
  const pipelineEndTime = formatTime(pipeline.finished_at);

  const stages = new Map();
  pipeline.runs.forEach((run) => {
    stages.set(run.stage, []);
  });
  const stagesKeys = [...stages.keys()];
  stagesKeys.forEach((stage) => {
    const stageRuns = pipeline.runs
      .filter((run) => run.stage === stage)
      .map((run) => [run.job_status, run.name]);
    stages.set(stage, stageRuns);
  });
  function updateCardColor(jobStatus: string) {
    if (jobStatus === JobStatus.Success) {
      return "success-job";
    } else if (jobStatus === JobStatus.Failed) {
      return "failed-job";
    } else {
      return "other-job-status";
    }
  }
</script>

<main>
  <div class="wrapper">
    <section aria-labelledby="pipeline-metadata-heading">
      <h2 id="pipeline-metadata-heading">Pipeline #{pipeline.pipeline_repo_id} metadata</h2>
      <a href={pipeline.project_repo_url}>{pipeline.project_name}</a>
      <a href={pipeline.pipeline_repo_url}>Pipeline #{pipeline.pipeline_repo_id}</a>
      <p><b>Pipeline duration:</b> {pipelineDuration}</p>
      <p><b>Pipeline started at:</b> {pipelineStartTime}</p>
      <p><b>Pipeline finished at:</b> {pipelineEndTime}</p>
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
  <section aria-labelledby="pipeline-jobs-flowchart-heading">
    <h2 id="pipeline-jobs-flowchart-heading">Pipeline jobs flowchart</h2>
    <ul class="pipeline-flowchart">
      {#each [...stages] as [stageName, runValues]}<li class="stage-name">
          {stageName}
          {#each runValues as run}
            <div class="run-card {updateCardColor(run[0])}">
              <ul>
                <li>Status: {run[0]}</li>
                <li>Job name: {run[1]}</li>
              </ul>
            </div>
          {/each}
        </li>
        <img id="arrow" src="/svg/right-arrow.svg" alt="Arrow going rightwards" />
      {/each}
    </ul>
  </section>
  <section class="aggregated-metric-values" aria-labelledby="aggregated-metric-values-heading">
    <h2 id="aggregated-metric-values-heading">Pipeline aggregated metric values</h2>
  </section>
</main>

<style>
  .pipeline-flowchart,
  .run-card li {
    list-style-type: none;
  }
  .pipeline-flowchart {
    display: flex;
    justify-content: space-evenly;
  }
  .run-card {
    display: flex;
    border: 1px solid black;
    padding: 12px;
    margin-bottom: 12px;
    width: 400px;
    height: 80px;
  }
  .run-card ul {
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
  }
  .success-job {
    background-color: #d0e3d0;
    color: black;
  }
  .failed-job {
    background-color: #ebc6c6;
    color: black;
  }
  .other-job-status {
    background-color: white;
    color: black;
  }
  #arrow {
    margin-top: 50px;
    height: 50px;
  }
  #arrow:last-of-type {
    display: none;
  }
</style>
