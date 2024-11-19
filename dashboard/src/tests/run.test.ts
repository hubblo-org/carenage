import { cleanup, render, screen, within } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { CiRun, Metric, Process } from "../types/carenage";
import processes from "./fixtures/run.json";
import Run from "../run.svelte";

describe("run test suite", () => {
  const run_id = "Run #8228228299";
  const processes_data: Process[] = processes.processes;

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
    const runDuration = screen.getByText(/180/i);
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
});
