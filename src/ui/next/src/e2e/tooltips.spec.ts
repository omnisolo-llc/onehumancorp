import { test, expect } from "@playwright/test";

test.describe("Tooltips", () => {
  test.beforeEach(async ({ page }) => {
    // Mock backend API responses for tooltips test
    await page.route("**/api/tooltips", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({
          "api-docs-tooltip":
            "Direct API access is only for custom integrations.",
          "settings-delivery-tooltip":
            "Turn this on to offer local delivery to your customers.",
        }),
      });
    });

    await page.route("**/api/api-docs-spec", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({
          openapi: "3.0.0",
          info: {
            title: "OHC Advanced API Reference",
            version: "1.0.0",
          },
          paths: {},
        }),
      });
    });
  });

  test("renders tooltip on hover", async ({ page }) => {
    // Navigate to a page that contains a tooltip
    await page.goto("/api-docs");

    // Locate the element with the tooltip text
    const tooltipTarget = page.locator("span", { hasText: "Advanced:" });

    // Wait for it to be visible
    await expect(tooltipTarget).toBeVisible();

    // Hover over the element
    await tooltipTarget.hover();

    // Wait for the tooltip text to appear
    const tooltipText = page
      .locator("div", {
        hasText: "Direct API access is only for custom integrations.",
      })
      .last();
    await expect(tooltipText).toBeVisible({ timeout: 5000 });

    // Move mouse away
    await page.mouse.move(0, 0);
  });

  test("renders settings tooltips on hover", async ({ page }) => {
    await page.goto("/settings");

    // Wait for the page to load
    await page.waitForLoadState("networkidle");

    // Verify the Delivery tooltip
    const deliveryToggle = page.locator("label", {
      hasText: "Enable Local Delivery",
    });
    await expect(deliveryToggle).toBeVisible();

    await deliveryToggle.hover();

    // Wait for the tooltip text to appear
    const deliveryTooltipText = page
      .locator("div", {
        hasText: "Turn this on to offer local delivery to your customers.",
      })
      .last();
    await expect(deliveryTooltipText).toBeVisible({ timeout: 5000 });

    // Move mouse away
    await page.mouse.move(0, 0);
  });

  test("handles mobile long-press and cancels on touchmove", async ({
    page,
  }) => {
    await page.goto("/api-docs");

    const tooltipTarget = page.locator("span", { hasText: "Advanced:" });
    await expect(tooltipTarget).toBeVisible();

    // Dispatch a touchstart event
    await tooltipTarget.dispatchEvent("touchstart");

    // Wait a bit to simulate holding (less than 500ms)
    await page.waitForTimeout(200);

    // Dispatch a touchmove event to simulate scrolling
    await tooltipTarget.dispatchEvent("touchmove");

    // Wait past the 500ms threshold
    await page.waitForTimeout(400);

    // The tooltip should NOT appear because touchmove cancelled it
    const tooltipText = page
      .locator("div", {
        hasText: "Direct API access is only for custom integrations.",
      })
      .last();
    await expect(tooltipText).not.toBeVisible();

    // Now test successful long press
    await tooltipTarget.dispatchEvent("touchstart");

    // Wait past the 500ms threshold
    await page.waitForTimeout(600);

    // The tooltip should appear
    await expect(tooltipText).toBeVisible();

    // End the touch
    await tooltipTarget.dispatchEvent("touchend");

    // After 2 seconds, it should disappear
    await page.waitForTimeout(2100);
    await expect(tooltipText).not.toBeVisible();
  });
});
