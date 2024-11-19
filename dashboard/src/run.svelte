<script lang="ts">
  import type { CiRun, Metric } from "./types/carenage";

  interface Props {
    run: CiRun;
  }

  const { run }: Props = $props();
  const numberOfRegisteredProcesses = run.processes.length;
  const metricsNames = run.processes[0].metrics.map((metric: Metric) => metric.metric_name);
</script>

<section>
  <div>
    <h1>Run {run.run_id}</h1>
    <a href="/{run.project_name}">Project {run.project_name}</a> >
    <a href="/{run.pipeline_id}">Pipeline {run.pipeline_id}</a>
  </div>
  <div>
    <p>Run executed in {run.job_duration}</p>
    <p>Job: {run.job_name}</p>
    <p>Processes registered: {numberOfRegisteredProcesses}</p>
    <p>Run status: {run.job_status}</p>
  </div>
</section>

<section>
  <label for="metric-name-select">Select a metric: </label>
  <select name="metric-names" id="metric-name-select">
    {#each metricsNames as metricName}
      <option>
        {metricName}
      </option>
    {/each}
  </select>
</section>

<style>
</style>
