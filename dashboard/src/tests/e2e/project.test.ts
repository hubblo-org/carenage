import { expect, test } from "@playwright/test";

const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";

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
