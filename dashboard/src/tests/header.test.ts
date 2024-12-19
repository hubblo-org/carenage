import { render, screen, within } from "@testing-library/svelte";
import { beforeEach, describe, expect, it } from "vitest";
import Header from "$lib/components/header.svelte";
import projectMetadata from "./fixtures/project_metadata.json";

describe("header test suite", () => {
  beforeEach(() => {
    render(Header, { props: { project: projectMetadata } });
  });
  it("displays a heading with the name of Carenage", () => {
    const carenageHeading = screen.getByRole("heading", { name: "Carenage" });
    expect(carenageHeading).toBeVisible();
  });
  it("displays the project's metadata", () => {
    /* Number of runs, of pipelines, project's repository, list of last pipelines with links*/
    const numberOfPipelines = projectMetadata.pipelines.length;
    const numberOfRuns = projectMetadata.pipelines
      .map((pipeline) => pipeline.runs.length)
      .reduce((acc, run) => run + acc, 0);
    const numberOfPipelinesText = screen.getByText(
      `Number of pipelines executed: ${numberOfPipelines}`
    );
    const numberOfRunsText = screen.getByText(`Number of job runs executed: ${numberOfRuns}`);
    const projectRepoLink = screen.getByRole("link", { name: `${projectMetadata.project_name}` });
    expect(numberOfPipelinesText).toBeVisible();
    expect(numberOfRunsText).toBeVisible();
    expect(projectRepoLink).toBeVisible();
  });
  it("displays a navigation element allowing the user to go to the project page showing its metadata and metrics", () => {
    const navbar = screen.getByRole("navigation");
    const projectPageLink = within(navbar).getByRole("link", {
      name: "Project summary and metrics"
    });
    expect(projectPageLink).toBeVisible();
  });

  // Not sure how to mock $page.data to test different behaviour for this component
  /* it("displays a link to the pipeline related to the run in the navigation element", () => {
    vi.mock("$app/stores", async () => {
      return { page: { data: { run: run } } };
    });
    const navbar = screen.getByRole("navigation");
    const pipelinePageLink = within(navbar).getByRole("link", {
      name: "Pipeline summary and metrics"
    });
    expect(pipelinePageLink).toBeVisible();
  }); */
});
