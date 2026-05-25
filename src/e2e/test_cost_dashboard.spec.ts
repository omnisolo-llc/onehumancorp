import { test, expect } from './fixtures';

test.describe('Cost Transparency Dashboard', () => {
  test('should display Cost & AI Usage dashboard with API mocked', async ({ page }) => {
    // Intercept the API call to return dummy data for Cost Dashboard
    await page.route('/api/billing/cost-dashboard', async route => {
      const json = {
        total_revenue: 10000,
        total_costs: 5000,
        llm_cost: 2000,
        storage_cost: 1000,
        payment_fees: 2000,
        period_start: "2024-05-01",
        period_end: "2024-05-31"
      };
      await route.fulfill({ json });
    });

    await page.goto('/');

    await page.evaluate(() => {
        // @ts-ignore
        showScreen('cost-dashboard-screen');
    });

    await expect(page.getByRole('heading', { name: 'Cost & AI Usage' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toHaveText('Total Costs: $50.00');
    await expect(page.locator('#cost-dashboard-llm')).toHaveText('LLM Usage: $20.00');
    await expect(page.locator('#cost-dashboard-storage')).toHaveText('Storage: $10.00');
    await expect(page.locator('#cost-dashboard-period')).toHaveText('Period: 2024-05-01 to 2024-05-31');
  });
});
