import { test, expect } from './fixtures';

test.describe('Documentation UI Verification', () => {
  test('Help portal videos render', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/help.html');

    // Make sure the title renders properly
    await expect(page.locator('h1')).toContainText('In-App Help Center');

    // Open floating widget
    const helpBtn = page.locator('#ohc-floating-help-btn');
    await helpBtn.waitFor({ state: 'visible' });
    await helpBtn.click();

    // Open videos tab
    const videosTab = page.locator('[data-target="tab-videos"]');
    await videosTab.waitFor({ state: 'visible' });
    await videosTab.click();

    // Verify video list is populated
    const videoList = page.locator('.ohc-help-content.active #video-list').first();
    await videoList.waitFor({ state: 'visible' });
    await expect(videoList).not.toBeEmpty();
    // Verify it isn't just loading text
    await expect(videoList).not.toContainText('Loading videos...', { timeout: 10000 });
  });

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
