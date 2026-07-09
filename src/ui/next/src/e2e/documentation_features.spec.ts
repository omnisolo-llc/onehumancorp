import { test, expect } from "@playwright/test";

test.describe("Documentation Features", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/changelog", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([
          {
            version: "v1.2.0",
            contentLines: ["### Feature", "- New Feature"],
            screenshot_url: "https://example.com/screenshot.png",
          },
        ]),
      });
    });
    await page.route("**/api/api-docs-spec", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ openapi: "3.0.0" }),
      });
    });
  });

  test("Changelog page loads correctly", async ({ page }) => {
    await page.goto("/changelog");
    await expect(page.getByTestId("changelog-title")).toBeVisible();
    await expect(page.getByText("Release Notes & Changelog")).toBeVisible();
  });

  test("API Docs page loads correctly", async ({ page }) => {
    await page.goto("/api-docs");
    await expect(page.getByTestId("api-docs-title")).toBeVisible();
    await expect(page.getByText("Advanced:")).toBeVisible();
  });

  test("Help Center and Chat opens", async ({ page }) => {
    await page.goto("/");
    const helpButton = page.locator("#ohc-floating-help-btn");
    await expect(helpButton).toBeVisible();
    await helpButton.click({ force: true });

    // Help Widget appears
    const askAnythingTab = page.getByText("Ask anything");
    await expect(askAnythingTab).toBeVisible();
    await askAnythingTab.click({ force: true });

    // Help chat widget
    await expect(page.locator("#ohc-floating-help-widget")).toBeVisible();
  });
});
