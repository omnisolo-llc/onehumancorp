import { test, expect } from './fixtures';

test.describe('Global Help Widget', () => {
  test('should be present and functional on dashboard', async ({ page }) => {
    await page.goto('/api/ui/dashboard.html');

    // The floating help button should be visible
    const helpBtn = page.locator('#ohc-floating-help-btn');
    await expect(helpBtn).toBeVisible();

    // Clicking it should open the chat widget
    await helpBtn.click();
    const chatWidget = page.locator('#ohc-floating-help-widget');
    await expect(chatWidget).toBeVisible();

    // Close the widget
    const closeBtn = page.locator('#ohc-floating-help-close');
    await closeBtn.click();
    await expect(chatWidget).not.toBeVisible();

    // Check that walkthrough can be triggered
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();
  });

  test('should be present and functional on POS', async ({ page }) => {
    await page.goto('/api/ui/pos.html');

    // The floating help button should be visible here too
    const helpBtn = page.locator('#ohc-floating-help-btn');
    await expect(helpBtn).toBeVisible();
  });
});
