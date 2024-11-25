import { render, screen } from "@testing-library/svelte";
import { beforeEach, describe, expect, it } from "vitest";
import Header from "$lib/components/header.svelte";
import run from "./fixtures/run.json";

describe("header test suite", () => {
  beforeEach(() => {
    render(Header, { props: { run: run } });
  });
  it("displays a heading with the name of Carenage", () => {
    const carenageHeading = screen.getByRole("heading", { name: "Carenage" });
    expect(carenageHeading).toBeVisible();
  });
  it("displays the project's metadata", () => {
    /* Number of runs, of pipelines, project's repository, list of last pipelines with links*/
    const numberOfPipelines = screen.getByText("Number of pipelines executed");
    const numberOfRuns = screen.getByText("Number of job runs executed");
    const projectRepoLink = screen.getByRole("link", { name: `${run.project_name}` });
    expect(numberOfPipelines).toBeVisible();
    expect(numberOfRuns).toBeVisible();
    expect(projectRepoLink).toBeVisible();
  });
});
