import { expect, test } from "@playwright/test";

const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
const pipelineId = 1520057997;
const runId = 8228228299;

test("renders the associated project page for the given id", async ({ context, page }) => {
  await context.addCookies([
    { name: "projectid", value: projectId, domain: "localhost", path: "/" }
  ]);
  await page.goto(`/projects/${projectId}`);
  const projectNameHeading = page.getByRole("heading", { name: "Project: hubblo/carenage" });
  const projectPipelinesTable = page.getByRole("table", {
    name: "List of executed CI pipelines for the project"
  });
  const projectRunsTable = page.getByRole("table", {
    name: "List of executed CI runs for the project"
  });
  await expect(projectNameHeading).toBeVisible();
  await expect(projectPipelinesTable).toBeVisible();
  await expect(projectRunsTable).toBeVisible();
});

test("routes to the selected pipeline page after clicking on the selected pipeline link", async ({
  context,
  page
}) => {
  await context.addCookies([
    { name: "projectid", value: projectId, domain: "localhost", path: "/" }
  ]);
  await page.goto(`/projects/${projectId}`);
  const projectPipelinesTable = page.getByRole("table", {
    name: "List of executed CI pipelines for the project"
  });
  const pipelineLink = projectPipelinesTable.getByRole("link", {
    name: `${pipelineId}`
  });
  await pipelineLink.click();

  await expect(
    page.getByRole("heading", {
      name: `Pipeline #${pipelineId} metadata`
    })
  ).toBeVisible();
});

test("routes to the selected run page after clicking on the selected run link", async ({
  context,
  page
}) => {
  await context.addCookies([
    { name: "projectid", value: projectId, domain: "localhost", path: "/" }
  ]);
  await page.goto(`/projects/${projectId}`);
  const projectRunsTable = page.getByRole("table", {
    name: "List of executed CI runs for the project"
  });
  const runLink = projectRunsTable.getByRole("link", { name: `${runId}` });
  await runLink.click();

  const metricSelectionHeading = page.getByRole("heading", {
    name: "Metric and process selection"
  });
  await expect(metricSelectionHeading).toBeVisible();
});

test("routes to a 404 page when navigating to a project route with invalid id", async ({
  page
}) => {
  await page.goto(`/projects/invalid-id`);
  const error404NotFound = page.getByText("404 Not found");
  await expect(error404NotFound).toBeVisible();
});
