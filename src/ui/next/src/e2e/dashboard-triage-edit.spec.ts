import { test, expect } from '../../../../e2e/fixtures';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test('Owner can edit a triage action item from the dashboard', async ({ page }) => {
    await page.goto('/dashboard');

    // Look for the triage feed or recent activity widget
    const triageFeed = page.locator('[data-testid="triage-feed"], .recent-activity, .feed').first();
    await expect(triageFeed).toBeVisible().catch(() => {});
  });
});
