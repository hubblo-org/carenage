import { cleanup, render, screen, within } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import userEvent from "@testing-library/user-event";
import type { CiRun, Metric, MetricValues, Process } from "../types/carenage";
import processes from "./fixtures/run.json";
import Run from "$lib/run.svelte";

describe("run test suite", () => {
  const run_id = "Run #8228228299";
  const processes_data: Array<Process> = processes.processes;

  const run: CiRun = {
    project_name: "hubblo/carenage",
    pipeline_id: "#1520057997",
    run_id: "#8228228299",
    job_name: "test_for_merge_request",
    job_status: "success",
    job_duration: 180,
    processes: processes_data
  };

  beforeEach(() => {
    render(Run, { props: { run: run } });
  });

  afterEach(() => {
    cleanup();
  });

  it("displays a heading with the run ID", () => {
    const runHeading = screen.getByRole("heading", { name: run_id });
    expect(runHeading).toBeVisible();
  });
  it("displays links to higher dimensions: to the pipeline related to the run, to the project related to the pipeline", () => {
    const pipelineLink = screen.getByRole("link", { name: /1520057997/i });
    const projectLink = screen.getByRole("link", { name: /hubblo\/carenage/i });
    expect(pipelineLink).toBeVisible();
    expect(projectLink).toBeVisible();
  });
  it("displays metadata on the processed run", () => {
    const runDuration = screen.getByText(/00:03:00/i);
    const numberOfProcesses = screen.getByText(/Processes registered: 15/i);
    const jobName = screen.getByText(/test_for_merge_request/i);
    const jobStatus = screen.getByText(/success/i);
    expect(runDuration).toBeVisible();
    expect(numberOfProcesses).toBeVisible();
    expect(jobName).toBeVisible();
    expect(jobStatus).toBeVisible();
  });
  it("displays a selection of metric names to choose from in order to display the metric values", () => {
    const metricsNames = processes_data[0].metrics.map((metric: Metric) => metric.metric_name);
    const metricNamesSelect = screen.getByRole("combobox", { name: /Select a metric/i });
    const { getAllByRole } = within(metricNamesSelect);
    const metricOptions = getAllByRole("option");
    metricOptions.map((metric_option, index) =>
      expect(metric_option).toHaveTextContent(metricsNames[index])
    );
  });
  it("displays a selection of processes to choose from in order to display the metric values", () => {
    const processesExes = processes_data.map((process: Process) => process.process.process_exe);
    const processesPids = processes_data.map((process: Process) =>
      process.process.process_pid.toString()
    );
    const processesSelect = screen.getByRole("combobox", { name: /Select a process/i });
    const { getAllByRole } = within(processesSelect);
    const processesOptions = getAllByRole("option");
    processesOptions.map((process_option, index) => {
      expect(process_option).toHaveTextContent(processesPids[index]);
      expect(process_option).toHaveTextContent(processesExes[index]);
    });
  });
  it("displays the metric values for the process and the metric_name selected by the user", async () => {
    const user = userEvent.setup();
    const isCpuAdpAvgImpact = (metric: Metric) => metric.metric_name === "cpu_adp_average_impact";
    const cpuAdpAvgImpactIndex = processes_data[0].metrics.findIndex(isCpuAdpAvgImpact);
    const pid64AvgPowerMeasuredValues = processes_data
      .filter((process: Process) => process.process.process_pid === 64)
      .map((process: Process) => process.metrics[cpuAdpAvgImpactIndex].metric_values);
    const processesSelect = screen.getByRole("combobox", { name: /Select a process/i });
    const metricNamesSelect = screen.getByRole("combobox", { name: /Select a metric/i });
    await user.selectOptions(processesSelect, ["64"]);
    await user.selectOptions(metricNamesSelect, ["cpu_adp_average_impact"]);

    // Might have to change those assertions depending on how values are to be represented and are accessible through the DOM.
    pid64AvgPowerMeasuredValues.map((metric_value: MetricValues, index) => {
      const timestamp = metric_value[index][0].toString();
      const value = metric_value[index][1].toString();
      const timestampText = screen.getByText(timestamp);
      const valueText = screen.getAllByText(value);
      expect(timestampText).toBeVisible();
      expect(valueText[0]).toBeVisible();
    });
  });
});
