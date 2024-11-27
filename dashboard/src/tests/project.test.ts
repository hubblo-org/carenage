import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { CiRun, ProjectRecord } from "../types/carenage";
import { cleanup, render, screen } from "@testing-library/svelte";
import Project from "$lib/components/project.svelte";
import runs from "./fixtures/runs.json";

describe("project component test suite", () => {
  const project: ProjectRecord = {
    project_id: "c2b04067-7c6d-40bb-a7e0-d094a9850196",
    project_name: "hubblo/carenage",
    created_at: "2024-06-13T18:10:14.658Z",
    runs: runs
  };

  beforeEach(() => {
    render(Project, { props: { project: project } });
  });
  afterEach(() => {
    cleanup();
  });
  it("displays metadata on the project", () => {
    const projectName = screen.getByRole("heading", { name: /hubblo\/carenage/i });
    const projectStartDate = screen.getByText(/6\/13\/2024/i);

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

    runs.map((run: CiRun) => {
      const runId = run.run_id;
      const runExecutionDate = run.started_at;
      const runIdCell = screen.getByRole("link", { name: runId.toString() });
      const runExecutionDateCell = screen.getByRole("cell", { name: runExecutionDate });
      expect(runIdCell).toBeVisible();
      expect(runExecutionDateCell).toBeVisible();
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

    // To modify according to API response that returns runs associated to a pipeline
    const pipelineExecutionDate = runs[0].started_at;
    const pipelineId = runs[0].pipeline_id;
    const pipelineIdCell = screen.getAllByRole("cell", { name: pipelineId.toString() });
    const pipelineExecutionDateCell = screen.getAllByRole("cell", { name: pipelineExecutionDate });
    expect(pipelineExecutionDateCell[0]).toBeVisible();
    expect(pipelineIdCell[0]).toBeVisible();
  });
});
