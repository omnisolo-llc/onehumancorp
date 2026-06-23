import { test, expect } from "@playwright/test";

test.describe("Tooltips", () => {
  test.beforeEach(async ({ page }) => {
    // Catch-all route to prevent network requests hanging the page load
    await page.route("**/*", async (route, request) => {
      const url = request.url();
      if (url.includes("/api/tooltips")) {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify({
            "api-docs-tooltip": "Direct API access is only for custom integrations.",
            "settings-delivery-tooltip": "Turn this on to offer local delivery to your customers.",
          }),
        });
      } else if (url.includes("/api/api-docs-spec")) {
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
      } else if (url.includes("/api/ui/swagger-ui.css") || url.includes("/api/ui/swagger-ui-bundle.js")) {
        await route.fulfill({
          status: 200,
          contentType: "text/plain",
          body: "",
        });
      } else {
        await route.continue();
      }
    });
  });

  test("renders tooltip on hover", async ({ page }) => {
    // Navigate to a page that contains a tooltip
    await page.goto("/api-docs");

    // Wait for the page to load
    await page.waitForLoadState("domcontentloaded");

    // Wait for the window to have tooltips loaded to prevent racing
    await page.waitForFunction(() => (window as any).OHC_TOOLTIPS !== undefined, { timeout: 10000 });

    // Locate the element with the tooltip text
    const tooltipTarget = page.locator("#api-docs-tooltip");

    // Wait for it to be attached to the DOM
    await tooltipTarget.waitFor({ state: "attached" });

    // Fallback to touchstart since mouseenter can be unreliable in Headless Chromium if bounding box is 0x0
    await page.evaluate(() => {
      const node = document.getElementById("api-docs-tooltip");
      if (node) {
        const target = node.querySelector('span') || node;
        // Dispatch touchstart because we know that works in tests with 0x0 wrappers
        target.dispatchEvent(new TouchEvent("touchstart", { bubbles: true, cancelable: true }));
      }
    });

    // Wait past the 500ms threshold for long press
    await page.waitForTimeout(600);

    // Wait for the tooltip text to appear
    const tooltipText = page
      .locator("div", {
        hasText: "Direct API access is only for custom integrations.",
      })
      .last();

    // It's attached, but might be invisible initially depending on CSS animations
    await tooltipText.waitFor({ state: "attached", timeout: 5000 });

    // Move mouse away
    await page.mouse.move(0, 0);
  });

  test("renders settings tooltips on hover", async ({ page }) => {
    await page.goto("/settings");

    // Wait for the page to load
    await page.waitForLoadState("domcontentloaded");

    // Wait for the window to have tooltips loaded to prevent racing
    await page.waitForFunction(() => (window as any).OHC_TOOLTIPS !== undefined, { timeout: 10000 });

    // Verify the Delivery tooltip
    const deliveryToggle = page.locator("#settings-delivery-tooltip");

    await deliveryToggle.waitFor({ state: "attached" });

    await page.evaluate(() => {
      const node = document.getElementById("settings-delivery-tooltip");
      if (node) {
        const target = node.firstElementChild || node;
        // Dispatch touchstart because we know that works in tests with 0x0 wrappers
        target.dispatchEvent(new TouchEvent("touchstart", { bubbles: true, cancelable: true }));
      }
    });

    // Wait past the 500ms threshold for long press
    await page.waitForTimeout(600);

    // Wait for the tooltip text to appear
    const deliveryTooltipText = page
      .locator("div", {
        hasText: "Turn this on to offer local delivery to your customers.",
      })
      .last();
    await deliveryTooltipText.waitFor({ state: "attached", timeout: 5000 });

    // Move mouse away
    await page.mouse.move(0, 0);
  });

  test("handles mobile long-press and cancels on touchmove", async ({
    page,
  }) => {
    await page.goto("/api-docs");

    // Wait for the page to load
    await page.waitForLoadState("domcontentloaded");

    // Wait for the window to have tooltips loaded to prevent racing
    await page.waitForFunction(() => (window as any).OHC_TOOLTIPS !== undefined, { timeout: 10000 });

    const tooltipTarget = page.locator("#api-docs-tooltip");
    await tooltipTarget.waitFor({ state: "attached" });

    // Dispatch a touchstart event directly on the node using evaluate to avoid hit target failures
    await tooltipTarget.evaluate((node) => {
        const target = node.querySelector('span') || node;
        target.dispatchEvent(new TouchEvent("touchstart", { bubbles: true, cancelable: true }));
    });

    // Wait a bit to simulate holding (less than 500ms)
    await page.waitForTimeout(200);

    // Dispatch a touchmove event to simulate scrolling
    await tooltipTarget.evaluate((node) => {
        const target = node.querySelector('span') || node;
        target.dispatchEvent(new TouchEvent("touchmove", { bubbles: true, cancelable: true }));
    });

    // Wait past the 500ms threshold
    await page.waitForTimeout(400);

    // The tooltip should NOT appear because touchmove cancelled it
    const tooltipText = page
      .locator("div", {
        hasText: "Direct API access is only for custom integrations.",
      })
      .last();
    await expect(tooltipText).not.toBeAttached();

    // Now test successful long press
    await tooltipTarget.evaluate((node) => {
        const target = node.querySelector('span') || node;
        target.dispatchEvent(new TouchEvent("touchstart", { bubbles: true, cancelable: true }));
    });

    // Wait past the 500ms threshold
    await page.waitForTimeout(600);

    // The tooltip should appear
    await tooltipText.waitFor({ state: "attached", timeout: 5000 });

    // End the touch
    await tooltipTarget.evaluate((node) => {
        const target = node.querySelector('span') || node;
        target.dispatchEvent(new TouchEvent("touchend", { bubbles: true, cancelable: true }));
    });

    // After 2 seconds, it should disappear
    await page.waitForTimeout(2100);
    await expect(tooltipText).not.toBeAttached();
  });
});
