import { test, expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/fixtures/accordion");
});

test("open and close the accordion", async ({ page }) => {
  const fixture = page.locator('[data-fixture="accordion-default"]');
  const toggle = fixture.locator("button").first();
  const content = fixture.getByText(/Content inside default accordion/);

  await expect(content).not.toBeVisible();

  await toggle.click();

  await expect(content).toBeVisible();
});
