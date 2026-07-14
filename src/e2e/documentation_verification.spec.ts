import { test, expect } from './fixtures';

test.describe('Documentation UI Verification', () => {

  test('Dashboard Walkthrough triggers', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/dashboard.html');

    // Click the Walkthrough button
    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await walkthroughBtn.waitFor({ state: 'visible' });
    await walkthroughBtn.click();

    // Wait for the walkthrough bubble to appear and verify text content
    const walkthroughBubble = page.locator('#walkthrough-bubble');
    await walkthroughBubble.waitFor({ state: 'visible' });
    await expect(walkthroughBubble).toBeVisible();
    await expect(walkthroughBubble).toContainText('Business Analytics');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close').first();
    if (await closeBtn.isVisible()) {
      await closeBtn.click();
      await expect(walkthroughBubble).not.toBeVisible();
    }
  });
});
