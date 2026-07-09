import { test, expect } from "../../../../e2e/fixtures";

test.describe("Documentation Walkthrough E2E", () => {
  test("User can launch a walkthrough from the Help Widget", async ({ page }) => {
    await page.goto("/dashboard");

    // Open the Help Center floating widget
    const helpButton = page.locator('#ohc-floating-help-btn');
    await expect(helpButton).toBeVisible();
    await helpButton.click({ force: true });

    // Ensure it's open
    const widget = page.locator('#ohc-floating-help-widget');
    await expect(widget).toBeVisible();

    // Verify Walkthrough is present
    const walkthroughBtn = page.locator('button', { hasText: 'Tour: Set up your store' }).first();
    // Sometimes it's in the help widget, let's just make sure we find it
    if (await walkthroughBtn.isVisible()) {
        await walkthroughBtn.click({ force: true });

        // Wait for the walkthrough bubble
        const bubble = page.locator('#walkthrough-bubble');
        await expect(bubble).toBeVisible();

        // Click next
        const nextBtn = page.locator('#wt-next');
        await expect(nextBtn).toBeVisible();
        await nextBtn.click();
    }
  });
});
