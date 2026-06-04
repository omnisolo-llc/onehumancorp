import { test, expect } from './fixtures';

test.describe('Cost Dashboard', () => {
  test('should display 7-Day Trend', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/cost-dashboard');

    // Wait for the cost dashboard screen to be visible
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();

    // Check that the trend container is visible
    const trendList = page.locator('#cost-dashboard-trend');
    await expect(trendList).toBeVisible();

    // We should eventually see the trend items populated (either 7 days of items or "No trend data")
    await expect(trendList.locator('li').first()).toBeVisible({ timeout: 10000 });
  });
});
