import { test, expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/fixtures/button");
});

test("button-default: shows 'not clicked' initially", async ({ page }) => {
  const fixture = page.locator('[data-fixture="button-default"]');
  const label = fixture.locator('[data-fixture="-clicked-label"]');
  await expect(label).toHaveText("not clicked");

  const button = fixture.locator("button");
  await button.click();

  await expect(label).toHaveText("clicked");
});
