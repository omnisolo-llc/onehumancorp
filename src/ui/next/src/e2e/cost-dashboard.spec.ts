import { test, expect } from '@playwright/test';

// NOTE: This test requires a docker-sandbox fix to run properly in CI
// due to pgvector pull permissions in the Bazel test sandbox environment.
test.describe('Cost Dashboard Loop', () => {

  test.beforeEach(async ({ page }) => {
    await page.route('**/api/billing/cost-dashboard', (route) => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          total_costs: 15000,
          total_revenue: 50000,
          bandwidth_savings: 1200,
          llm_cost: 5000,
          storage_cost: 2000,
          payment_fees: 1500,
          network_cost: 6500,
          cache_hit_rate: 85.5,
          cost_per_1k_tokens: 0.0015,
          period_start: "2024-05-01",
          period_end: "2024-05-31",
          trend: [
            { date: "2024-05-25", total_cost: 500 },
            { date: "2024-05-26", total_cost: 600 }
          ]
        })
      });
    });
  });

  test('Cost dashboard loads and displays data', async ({ page }) => {
    // Navigate to the dashboard page
    await page.goto('/cost-dashboard');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 10000 });

    // Check that the Advisory Summary is present
    await expect(page.locator('h2', { hasText: 'Advisory Summary' })).toBeVisible();

    // Check that the Cost Transparency section is present
    await expect(page.locator('h2', { hasText: 'Cost Transparency' })).toBeVisible();

    // Check that Total Costs is displayed
    await expect(page.locator('h2', { hasText: 'Total Costs' })).toBeVisible();

    // Check that Cost Breakdown section is present
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();

    // Check for individual breakdown items
    await expect(page.locator('span', { hasText: 'LLM Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' })).toBeVisible();

    // Check navigation works
    await page.locator('button', { hasText: 'Back to My Plan' }).click();
    await expect(page).toHaveURL('/plan');
  });
});
