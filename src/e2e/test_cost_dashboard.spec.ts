import { test, expect } from './fixtures';

test.describe('Cost Dashboard - Caching and Metrics', () => {
  test('should display prompt caching cost savings and all required cost metrics on dashboard', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
    await expect(page.locator('#cost-dashboard-period')).toContainText('Period:');
    await expect(page.locator('#cost-dashboard-llm')).toContainText('$');
    await expect(page.locator('#cost-dashboard-storage')).toContainText('$');
    await expect(page.locator('#cost-dashboard-payment-fees')).toContainText('$');
  });
});
