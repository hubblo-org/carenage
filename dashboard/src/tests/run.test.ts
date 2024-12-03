import { cleanup, render, screen, within } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import userEvent from "@testing-library/user-event";
import type { Metric, MetricValues, Process } from "$lib/types/carenage";
import runData from "./fixtures/run.json";
import Run from "$lib/components/run.svelte";

describe("run test suite", () => {
  const runId = "Run 8228228299";
  const processesData: Array<Process> = runData.processes;

  beforeEach(() => {
    render(Run, { props: { run: runData } });
  });

  afterEach(() => {
    cleanup();
  });

  it("displays a heading with the run ID", () => {
    const runHeading = screen.getByRole("heading", { name: runId });
    expect(runHeading).toBeVisible();
  });
  it("displays links to the repository page, to the pipeline logs related to the run, to the run logs", () => {
    const pipelineLink = screen.getByRole("link", { name: /1520057997/i });
    const projectLink = screen.getByRole("link", { name: /hubblo\/carenage/i });
    const runLink = screen.getByRole("link", { name: /8228228299/i });

    expect(pipelineLink).toBeVisible();
    expect(projectLink).toBeVisible();
    expect(runLink).toBeVisible();
  });
  it("displays metadata on the processed run", () => {
    const runDuration = screen.getByText(/00:03:00/i);
    const numberOfProcesses = screen.getByText(/Processes registered/i);
    const jobName = screen.getByText(/test_for_merge_request/i);
    const jobStatus = screen.getByText(/success/i);
    expect(runDuration).toBeVisible();
    expect(numberOfProcesses).toBeVisible();
    expect(jobName).toBeVisible();
    expect(jobStatus).toBeVisible();
  });
  it("displays a selection of metric names to choose from in order to display the metric values", () => {
    const metricSelectionLabel = "Metric and process selection";
    const metricSelectionHeading = screen.getByRole("heading", {
      name: metricSelectionLabel
    });
    const metricSelectionSection = screen.getByRole("region", {
      name: metricSelectionLabel
    });
    const metricsNames = processesData[0].metrics.map((metric: Metric) => metric.metric_name);
    const metricNamesSelect = screen.getByRole("combobox", { name: /Select a metric/i });
    const { getAllByRole } = within(metricNamesSelect);
    const metricOptions = getAllByRole("option");

    expect(metricSelectionHeading).toBeVisible();
    expect(metricSelectionSection).toBeVisible();
    metricOptions.map((metric_option, index) =>
      expect(metric_option).toHaveTextContent(metricsNames[index])
    );
  });
  it("displays a selection of processes to choose from in order to display the metric values", () => {
    const processesExes = processesData.map((process: Process) => process.process.process_exe);
    const processesPids = processesData.map((process: Process) =>
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
    const isCpuAdpAvgImpact = (metric: Metric) =>
      metric.metric_name === "cpu_adp_average_impact_kgsbeq";
    const cpuAdpAvgImpactIndex = processesData[0].metrics.findIndex(isCpuAdpAvgImpact);
    const pid64CpuAvgAdpValues = processesData
      .filter((process: Process) => process.process.process_pid === 64)
      .map((process: Process) => process.metrics[cpuAdpAvgImpactIndex].metric_values);

    const metricValuesLabel = "Metric values";
    const metricValuesHeading = screen.getByRole("heading", { name: metricValuesLabel });
    const metricValuesSection = screen.getByRole("region", { name: metricValuesLabel });
    const processesSelect = screen.getByRole("combobox", { name: /Select a process/i });
    const metricNamesSelect = screen.getByRole("combobox", { name: /Select a metric/i });
    await user.selectOptions(processesSelect, ["64"]);
    await user.selectOptions(metricNamesSelect, ["cpu_adp_average_impact_kgsbeq"]);

    const metricsTable = screen.getByRole("table", {
      name: "Values for selected process and metric"
    });
    const timestampsColumn = screen.getByRole("columnheader", { name: "Timestamp" });
    const metricValuesColumn = screen.getByRole("columnheader", { name: /Metric value/i });

    expect(metricValuesHeading).toBeVisible();
    expect(metricValuesSection).toBeVisible();
    expect(metricsTable).toBeVisible();
    expect(timestampsColumn).toBeVisible();
    expect(metricValuesColumn).toBeVisible();

    pid64CpuAvgAdpValues.map((metric_value: MetricValues, index) => {
      const timestamp = metric_value[index][0].toString();
      const value = metric_value[index][1].toString();
      const timestampText = screen.getByRole("cell", { name: timestamp });
      const valueText = screen.getAllByRole("cell", { name: value });
      expect(timestampText).toBeVisible();
      expect(valueText[0]).toBeVisible();
    });
  });
  it("displays an accessible element where a graph will be generated from the values of the process and metric_name selected by the user", async () => {
    const graph = screen.getByRole("img", { name: "Metric values distributed on a graph" });
    expect(graph).toBeVisible();
  });
  it("displays a summary of the metric values for the process and metric_name selected by the user", async () => {
    const isAvgPowerMeasured = (metric: Metric) =>
      metric.metric_name === "average_power_measured_w";
    const avgPowerMeasuredIndex = processesData[0].metrics.findIndex(isAvgPowerMeasured);
    const pid53AvgPowerMeasuredValues = processesData
      .filter((process: Process) => process.process.process_pid === 53)
      .map((process: Process) => process.metrics[avgPowerMeasuredIndex].metric_values);

    const values = pid53AvgPowerMeasuredValues[0].map((value) => value[1]);
    const maxValue = Math.max.apply(null, values);
    const average = (array: number[]) =>
      array.reduce((a: number, b: number) => a + b) / array.length;
    const avgValue = average(values);
    const minValue = Math.min.apply(null, values);
    const metricsTable = screen.getByRole("table", {
      name: "Values for selected process and metric"
    });
    const maxValueText = within(metricsTable).getByRole("rowheader", {
      name: `Highest value: ${maxValue}`
    });
    const minValueText = within(metricsTable).getByRole("rowheader", {
      name: `Smallest value: ${minValue}`
    });
    const avgValueText = within(metricsTable).getByRole("rowheader", {
      name: `Average value: ${avgValue}`
    });
    expect(maxValueText).toBeVisible();
    expect(minValueText).toBeVisible();
    expect(avgValueText).toBeVisible();
  });
});
