<script lang="ts">
  import type { CiRun, Metric, MetricValues, Process } from "$lib/types/carenage";
  import { onMount } from "svelte";
  import { createGraph } from "$lib/create-graph.svelte";
  import { average, formatDuration, getMetricUnit } from "$lib/utils.js";

  interface Props {
    run: CiRun;
  }

  const { run }: Props = $props();
  const numberOfRegisteredProcesses = run.processes.length;
  const metricsNames = run.processes[0].metrics.map((metric: Metric) => metric.metric_name);

  let processSelected = $state(run.processes[0].process.process_pid);
  let metricSelected = $state(metricsNames[0]);
  let formattedRunDuration = formatDuration(run.duration);
  let metricUnit = $derived(getMetricUnit(metricSelected));

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

  let summaryValues = $derived.by(() => {
    const values = metricValues.map((value: MetricValues) => value[1]);
    const maxValue = Math.max.apply(null, values);
    const minValue = Math.min.apply(null, values);
    const avgValue = average(values);
    return [maxValue, avgValue, minValue];
  });

  onMount(async () => {
    await createGraph(metricValues, metricSelected);
  });
</script>

<main>
  <div class="wrapper">
    <section>
      <div class="run-metadata">
        <h2>Run {run.run_repo_id}</h2>
        <a href={run.project_repo_url}>Project {run.project_name}</a>
        <a href={run.pipeline_repo_url}>Pipeline #{run.pipeline_repo_id} </a>
        <a href={run.run_repo_url}> Run #{run.run_repo_id}</a>
      </div>
      <div>
        <p>Run executed in {formattedRunDuration}</p>
        <p><b>Job</b>: {run.job_name}</p>
        <p><b>Processes registered</b>: {numberOfRegisteredProcesses}</p>
        <p><b>Run status</b>: {run.job_status}</p>
      </div>
    </section>
    <section class="metric-selection" aria-labelledby="metric-selection-heading">
      <h2 id="metric-selection-heading">Metric and process selection</h2>
      <label for="metric-name-select">Select a metric: </label>
      <select
        name="metric-names"
        id="metric-name-select"
        bind:value={metricSelected}
        onchange={() => createGraph(metricValues, metricSelected)}
      >
        {#each metricsNames as metricName}
          <option value={metricName}>
            {metricName}
          </option>
        {/each}
      </select>
      <label for="process-select">Select a process: </label>
      <select
        name="processes"
        id="process-select"
        bind:value={processSelected}
        onchange={() => createGraph(metricValues, metricSelected)}
      >
        {#each run.processes as process}
          <option value={process.process.process_pid}>
            PID #{process.process.process_pid}
            {process.process.process_exe}
          </option>
        {/each}
      </select>
    </section>

    <section class="metric-values" aria-labelledby="metric-values-heading">
      <h2 id="metric-values-heading">Metric values</h2>
      <table>
        <caption>Values for selected process and metric</caption>
        <thead>
          <tr><th scope="row" colspan="2"> Highest value: {summaryValues[0]} </th></tr>
          <tr><th scope="row" colspan="2"> Average value: {summaryValues[1]} </th></tr>
          <tr><th scope="row" colspan="2"> Smallest value: {summaryValues[2]} </th></tr>
          <tr><th scope="col">Timestamp</th><th scope="col">Metric value (in {metricUnit})</th></tr
          ></thead
        >
        <tbody>
          {#each metricValues as metricValue}
            <tr>
              <td>{metricValue[0]}</td>
              <td>{metricValue[1]}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </section>
  </div>
  <div id="graph" aria-label="Metric values distributed on a graph" role="img"></div>
</main>

<style>
  .run-metadata {
    display: flex;
    gap: 5px;
    flex-direction: column;
  }
  .metric-values {
    overflow: auto;
  }
  .metric-selection {
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
  }
  .metric-selection label {
    width: 40%;
    color: white;
    background-color: black;
  }
  .metric-selection:hover label {
    color: black;
    background-color: white;
  }
</style>
