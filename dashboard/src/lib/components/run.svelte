<script lang="ts">
  import type { CiRun, Metric, MetricValues, Process } from "../types/carenage";
  import { onMount } from "svelte";
  import { createGraph } from "$lib/create-graph.svelte";
  import { formatDuration } from "$lib/utils.js";

  interface Props {
    run: CiRun;
  }

  const { run }: Props = $props();
  const numberOfRegisteredProcesses = run.processes.length;
  const metricsNames = run.processes[0].metrics.map((metric: Metric) => metric.metric_name);

  let processSelected = $state(run.processes[0].process.process_pid);
  let metricSelected = $state(metricsNames[0]);
  let formatted_run_duration = formatDuration(run.job_duration);

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

  onMount(async () => {
    await createGraph(metricValues);
    console.log("mounted");
  });
</script>

<main>
  <div class="wrapper">
    <section>
      <div>
        <h1>Run {run.run_id}</h1>
        <a href="/{run.project_name}">Project {run.project_name}</a> >
        <a href="/{run.pipeline_id}">Pipeline {run.pipeline_id}</a>
      </div>
      <div>
        <p>Run executed in {formatted_run_duration}</p>
        <p><b>Job</b>: {run.job_name}</p>
        <p><b>Processes registered</b>: {numberOfRegisteredProcesses}</p>
        <p><b>Run status</b>: {run.job_status}</p>
      </div>
    </section>
    <section class="metric-selection">
      <label for="metric-name-select">Select a metric: </label>
      <select
        name="metric-names"
        id="metric-name-select"
        bind:value={metricSelected}
        onchange={() => createGraph(metricValues)}
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
        onchange={() => createGraph(metricValues)}
      >
        {#each run.processes as process}
          <option value={process.process.process_pid}>
            PID #{process.process.process_pid}
            {process.process.process_exe}
          </option>
        {/each}
      </select>
    </section>

    <section class="values">
      <h3>Metric values</h3>
      {#each metricValues as metricValue}
        <p>Timestamp</p>
        <p>{metricValue[0]}</p>
        <p>Value</p>
        <p>{metricValue[1]}</p>
      {/each}
    </section>
  </div>
  <div id="graph" aria-label="Metric values distributed on a graph" role="img"></div>
</main>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
  }
  .values {
    overflow: auto;
    height: 200px;
  }
  .metric-selection {
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
  }
  .metric-selection label {
    width: 30%;
    color: white;
    background-color: black;
  }
</style>
