import { expect, test } from "@playwright/test";

test("routes to a 404 page when navigating to a project route with invalid id", async ({
  page
}) => {
  await page.goto(`/pipelines/invalid-id`);
  const error404NotFound = page.getByText("404 Not found");
  await expect(error404NotFound).toBeVisible();
});
