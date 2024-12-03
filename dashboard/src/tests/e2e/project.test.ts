import { expect, test } from "@playwright/test";

const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
const pipelineId = 1520057997;
const runId = 8228228299;

test("renders the associated project page for the given id", async ({ page }) => {
  await page.goto(`/projects/${projectId}`);
  const projectNameHeading = page.getByRole("heading", { name: /hubblo\/carenage/i });
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
  page
}) => {
  await page.goto(`/projects/${projectId}`);
  const projectPipelinesTable = page.getByRole("table", {
    name: "List of executed CI pipelines for the project"
  });
  const pipelineLink = projectPipelinesTable.getByRole("link", {
    name: `${pipelineId}`
  });
  await pipelineLink.click();

  const pipelineMetadataHeading = page.getByRole("heading", {
    name: `Pipeline #${pipelineId} metadata`
  });
  await expect(pipelineMetadataHeading).toBeVisible();
});

test("routes to the selected run page after clicking on the selected run link", async ({
  page
}) => {
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
