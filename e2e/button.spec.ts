import { test, expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/fixtures/button");
});

const fixtures = [
  "button-with-axis",
  "button-with-boolean-intersect",
  "button-with-boolean-subtract",
  "button-with-boolean-union",
  "button-with-chamfer",
  "button-with-cube",
  "button-with-delete",
  "button-with-dimension",
  "button-with-duplicate",
  "button-with-export",
  "button-with-extrude",
  "button-with-fillet",
  "button-with-grid-snap",
  "button-with-group",
  "button-with-import",
  "button-with-layers",
  "button-with-mirror",
  "button-with-move",
  "button-with-orbit",
  "button-with-redo",
  "button-with-rotate",
  "button-with-scale",
  "button-with-section-cut",
  "button-with-select",
  "button-with-sketch",
  "button-with-solid-view",
  "button-with-undo",
  "button-with-wireframe",
  "button-with-zoom-fit",
];

for (const fixture of fixtures) {
  test(`${fixture} matches snapshot`, async ({ page }) => {
    const el = page.locator(`[data-fixture="${fixture}"]`);
    await expect(el).toBeVisible();
    await expect(el).toHaveScreenshot(`${fixture}.png`);
  });
}

test("button-default: shows 'not clicked' initially", async ({ page }) => {
  const fixture = page.locator('[data-fixture="button-default"]');
  const label = fixture.locator('[data-fixture="-clicked-label"]');
  await expect(label).toHaveText("not clicked");

  const button = fixture.locator("button");
  await button.click();

  await expect(label).toHaveText("clicked");
});
