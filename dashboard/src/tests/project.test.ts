import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { CiPipelineMetadata, CiRunMetadata } from "$lib/types/carenage";
import { cleanup, render, screen, within } from "@testing-library/svelte";
import Project from "$lib/components/project.svelte";
import project_metadata from "./fixtures/project_metadata.json";

describe("project component test suite", () => {
  beforeEach(() => {
    render(Project, { props: { project: project_metadata } });
  });
  afterEach(() => {
    cleanup();
  });
  it("displays metadata on the project", () => {
    const projectMetadataSection = screen.getByRole("region", { name: /hubblo\/carenage/i });
    const projectName = screen.getByRole("heading", { name: /hubblo\/carenage/i });
    const projectStartDate = screen.getByText(/6\/13\/2024/i);

    expect(projectMetadataSection).toBeVisible();
    expect(projectName).toBeVisible();
    expect(projectStartDate).toBeVisible();
  });
  it("displays a list of runs executed, with each run element allowing the user to navigate to the run's metrics", () => {
    const runsTable = screen.getByRole("table", {
      name: "List of executed CI runs for the project"
    });
    const runIdsColumn = screen.getByRole("columnheader", { name: "Run ID" });
    const runExecutionDateColumn = screen.getByRole("columnheader", {
      name: "Run date of execution"
    });
    expect(runsTable).toBeVisible();
    expect(runIdsColumn).toBeVisible();
    expect(runExecutionDateColumn).toBeVisible();

    project_metadata.pipelines.forEach((pipeline: CiPipelineMetadata) => {
      const pipelineId = pipeline.pipeline_repo_id;
      const pipelineIdCell = within(runsTable).getByRole("cell", {
        name: `Pipeline ID #${pipelineId.toString()}`
      });
      expect(pipelineIdCell).toBeVisible();
      pipeline.runs.forEach((run: CiRunMetadata) => {
        const runId = run.run_repo_id;
        const runExecutionDate = run.started_at;
        const runIdCell = within(runsTable).getByRole("link", { name: runId.toString() });
        const runExecutionDateCell = within(runsTable).getByRole("cell", {
          name: runExecutionDate
        });
        expect(runIdCell).toBeVisible();
        expect(runExecutionDateCell).toBeVisible();
      });
    });
  });
  it("displays a list of pipelines executed, with each pipeline element allowing the user to navigate to the pipeline's metrics", () => {
    const pipelinesTable = screen.getByRole("table", {
      name: "List of executed CI pipelines for the project"
    });
    const pipelineIdsColumn = screen.getByRole("columnheader", { name: "Pipeline ID" });
    const pipelineExecutionDateColumn = screen.getByRole("columnheader", {
      name: "Pipeline date of execution"
    });

    expect(pipelinesTable).toBeVisible();
    expect(pipelineIdsColumn).toBeVisible();
    expect(pipelineExecutionDateColumn).toBeVisible();

    project_metadata.pipelines.forEach((pipeline: CiPipelineMetadata) => {
      const pipelineExecutionDate = pipeline.started_at;
      const pipelineId = pipeline.pipeline_repo_id;
      const pipelineIdCell = screen.getAllByRole("link", { name: pipelineId.toString() });
      const pipelineExecutionDateCell = screen.getAllByRole("cell", {
        name: pipelineExecutionDate
      });
      expect(pipelineExecutionDateCell[0]).toBeVisible();
      expect(pipelineIdCell[0]).toBeVisible();
    });
  });
  it("displays a section with the aggregated metrics for the project, where the user can select a metric to see the associated aggregated values", () => {
    const aggregatedMetricValuesLabel = "Project's aggregated metric values";
    const aggregatedMetricValuesHeading = screen.getByRole("heading", {
      name: aggregatedMetricValuesLabel
    });
    const aggregatedMetricValuesSection = screen.getByRole("region", {
      name: aggregatedMetricValuesLabel
    });
    expect(aggregatedMetricValuesSection).toBeVisible();
    expect(aggregatedMetricValuesHeading).toBeVisible();
  });
});
