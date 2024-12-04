import { render, screen } from "@testing-library/svelte";
import { beforeEach, describe, expect, it } from "vitest";
import Header from "$lib/components/header.svelte";
import project_metadata from "./fixtures/project_metadata.json";

describe("header test suite", () => {
  beforeEach(() => {
    render(Header, { props: { project: project_metadata } });
  });
  it("displays a heading with the name of Carenage", () => {
    const carenageHeading = screen.getByRole("heading", { name: "Carenage" });
    expect(carenageHeading).toBeVisible();
  });
  it("displays the project's metadata", () => {
    /* Number of runs, of pipelines, project's repository, list of last pipelines with links*/
    const numberOfPipelines = project_metadata.pipelines.length;
    const numberOfRuns = project_metadata.pipelines
      .map((pipeline) => pipeline.runs.length)
      .reduce((acc, run) => run + acc, 0);
    const numberOfPipelinesText = screen.getByText(
      `Number of pipelines executed: ${numberOfPipelines}`
    );
    const numberOfRunsText = screen.getByText(`Number of job runs executed: ${numberOfRuns}`);
    const projectRepoLink = screen.getByRole("link", { name: `${project_metadata.project_name}` });
    expect(numberOfPipelinesText).toBeVisible();
    expect(numberOfRunsText).toBeVisible();
    expect(projectRepoLink).toBeVisible();
  });
});
