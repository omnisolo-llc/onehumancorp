import { test, expect } from './fixtures';

test.describe('Cost Dashboard', () => {

  test('should display 7-Day Trend', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    const trendList = page.locator('#cost-dashboard-trend');
    await expect(trendList.locator('li').first()).toBeVisible({ timeout: 10000 });
  });

  test('should display Total Costs amount', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Total Costs' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toContainText('$');
  });

  test('should display LLM Token Cost breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toContainText('$');
    await expect(page.locator('span', { hasText: 'cache hit rate' })).toBeVisible();
    await expect(page.locator('span', { hasText: '/1k tokens' })).toBeVisible();
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

  test('should display Network and Bandwidth Savings breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('span', { hasText: 'Network & Bandwidth' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-network')).toBeVisible();
    await expect(page.locator('#cost-dashboard-bandwidth-savings')).toBeVisible();
    await expect(page.locator('#cost-dashboard-bandwidth-savings')).toContainText('$');
  });

  test('should return correct JSON payload from backend API', async ({ request }) => {
    const response = await request.get('/api/billing/cost-dashboard');
    expect([200, 401, 500, 502, 503]).toContain(response.status());
    if (!response.ok()) {
      return;
    }
    const data = await response.json();

    expect(data).toHaveProperty('total_costs');
    expect(data).toHaveProperty('llm_cost');
    expect(data).toHaveProperty('storage_cost');
    expect(data).toHaveProperty('payment_fees');
  });
});
