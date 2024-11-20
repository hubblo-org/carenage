<script lang="ts">
  import type { CiRun, Metric, MetricValues, Process } from "./types/carenage";
  import format_duration from "$lib/utils.js";

  interface Props {
    run: CiRun;
  }

  const { run }: Props = $props();
  const numberOfRegisteredProcesses = run.processes.length;
  const metricsNames = run.processes[0].metrics.map((metric: Metric) => metric.metric_name);

  let processSelected = $state(run.processes[0].process.process_pid);
  let metricSelected = $state(metricsNames[0]);
  let formatted_run_duration = format_duration(run.job_duration);

  let metricValues = $derived.by(() => {
    const isMetricSelected = (metric: Metric) => metric.metric_name === metricSelected;
    const processMetrics = run.processes
      .filter((process: Process) => process.process.process_pid === processSelected)
      .map((process: Process) => process.metrics);
    const metricIndex = processMetrics[0].findIndex(isMetricSelected);
    const metricValues: MetricValues = processMetrics.map(
      (metrics: Metric[]) => metrics[metricIndex]
    )[0].metric_values;
    return metricValues;
  });
</script>

<section>
  <div>
    <h1>Run {run.run_id}</h1>
    <a href="/{run.project_name}">Project {run.project_name}</a> >
    <a href="/{run.pipeline_id}">Pipeline {run.pipeline_id}</a>
  </div>
  <div>
    <p>Run executed in {formatted_run_duration}</p>
    <p>Job: {run.job_name}</p>
    <p>Processes registered: {numberOfRegisteredProcesses}</p>
    <p>Run status: {run.job_status}</p>
  </div>
</section>

<section>
  <label for="metric-name-select">Select a metric: </label>
  <select name="metric-names" id="metric-name-select" bind:value={metricSelected}>
    {#each metricsNames as metricName}
      <option value={metricName}>
        {metricName}
      </option>
    {/each}
  </select>
  <label for="process-select">Select a process: </label>
  <select name="processes" id="process-select" bind:value={processSelected}>
    {#each run.processes as process}
      <option value={process.process.process_pid}>
        PID #{process.process.process_pid}
        {process.process.process_exe}
      </option>
    {/each}
  </select>
</section>
<section>
  {#each metricValues as metricValue}
    <p>{metricValue[0]}</p>
    <p>{metricValue[1]}</p>
  {/each}
</section>

<style>
</style>
