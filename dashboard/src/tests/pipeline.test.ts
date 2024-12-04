import { render, cleanup, screen, within } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { formatTime, formatDuration } from "$lib/utils";
import Pipeline from "$lib/components/pipeline.svelte";
import pipeline from "./fixtures/pipeline.json";
import type { CiRunMetadata } from "$lib/types/carenage";

describe("pipeline test suite", () => {
  beforeEach(() => render(Pipeline, { props: { pipeline: pipeline } }));
  afterEach(() => cleanup());
  it("displays to the user a section with links to the project page and pipeline logs", () => {
    const pipelineMetadataLabel = `Pipeline #${pipeline.pipeline_repo_id} metadata`;
    const pipelineMetadataSection = screen.getByRole("region", { name: pipelineMetadataLabel });
    const pipelineHeading = screen.getByRole("heading", {
      name: pipelineMetadataLabel
    });
    const pipelineLink = within(pipelineMetadataSection).getByRole("link", { name: /1520057997/i });
    const projectLink = within(pipelineMetadataSection).getByRole("link", {
      name: /hubblo\/carenage/i
    });

    expect(pipelineMetadataSection).toBeVisible();
    expect(pipelineHeading).toBeVisible();
    expect(pipelineLink).toBeVisible();
    expect(projectLink).toBeVisible();
  });
  it("displays to the user a selection of metadata related to the pipeline", () => {
    const pipelineMetadataLabel = `Pipeline #${pipeline.pipeline_repo_id} metadata`;
    const pipelineMetadataSection = screen.getByRole("region", { name: pipelineMetadataLabel });
    const formattedPipelineDuration = formatDuration(pipeline.duration);
    const formattedPipelineStartDate = formatTime(pipeline.started_at);
    const formattedPipelineEndDate = formatTime(pipeline.finished_at);

    const pipelineDuration = within(pipelineMetadataSection).getByText(
      `${formattedPipelineDuration}`,
      { exact: false }
    );
    const pipelineStartDate = within(pipelineMetadataSection).getByText(
      `${formattedPipelineStartDate}`,
      { exact: false }
    );
    const pipelineEndDate = within(pipelineMetadataSection).getByText(
      `${formattedPipelineEndDate}`,
      { exact: false }
    );
    expect(pipelineDuration).toBeVisible();
    expect(pipelineStartDate).toBeVisible();
    expect(pipelineEndDate).toBeVisible();
  });
  it("displays to the user a section with links to the executed runs for the given pipeline", () => {
    const runsTableLabel = `Executed runs for pipeline #${pipeline.pipeline_repo_id}`;
    const runsTableSection = screen.getByRole("region", { name: runsTableLabel });
    const runsTableHeading = screen.getByRole("heading", { name: runsTableLabel });

    const runsTable = within(runsTableSection).getByRole("table", {
      name: "List of executed CI runs for the pipeline"
    });
    const runIdColumn = within(runsTable).getByRole("columnheader", { name: "Run ID" });
    const runDateOfExecutionColumn = within(runsTable).getByRole("columnheader", {
      name: "Run start time of execution"
    });
    expect(runsTableHeading).toBeVisible();
    expect(runIdColumn).toBeVisible();
    expect(runDateOfExecutionColumn).toBeVisible();

    pipeline.runs.forEach((run: CiRunMetadata) => {
      const formattedPipelineStartTime = formatTime(run.started_at);
      const runIdCell = within(runsTable).getByRole("link", {
        name: `Run #${run.run_repo_id.toString()}`
      });
      const runDateOfExecutionCell = within(runsTable).getByRole("cell", {
        name: formattedPipelineStartTime
      });
      expect(runIdCell).toBeVisible();
      expect(runDateOfExecutionCell).toBeVisible();
    });
  });
  it("displays to the user a section with the aggregated metrics for the pipeline, where the user can choose a metric to be displayed", () => {
    const aggregatedMetricValuesLabel = "Pipeline aggregated metric values";
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
