import { test, expect } from './fixtures';

test.describe('Cost Dashboard', () => {
  test('should display 7-Day Trend', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    const trendList = page.locator('#cost-dashboard-trend');
    await expect(trendList).toBeVisible();
    await expect(trendList.locator('li').first()).toBeVisible({ timeout: 10000 });
  });

  test('should display Total Costs amount', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toContainText('$');
  });

  test('should display LLM Token Cost breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toContainText('$');
  });

  test('should display Storage and CDN Cost breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toContainText('$');
  });

  test('should display Payment Processor Fees breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toContainText('$');
  });
});
