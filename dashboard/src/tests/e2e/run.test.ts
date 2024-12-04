import { expect, test } from "@playwright/test";

const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
const pipelineId = "d199e857-fb0f-46b1-9846-74e53b494740";
const runGitlabId = 8228228299;

test("renders header with a link to the related pipeline for the displayed run page", async ({
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
  const runLink = projectRunsTable.getByRole("link", { name: `${runGitlabId}` });
  await runLink.click();

  const pipelineLink = page.getByRole("link", { name: "Pipeline summary and metrics" });
  await expect(pipelineLink).toBeVisible();
  await expect(pipelineLink).toHaveAttribute("href", `/pipelines/${pipelineId}`);
});
