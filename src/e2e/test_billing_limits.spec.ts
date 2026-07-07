import { test, expect } from './fixtures';

test('Cost Soft Limit friendly prompt shows', async ({ page, loginAs, unlimitedAdminUser }) => {
  await loginAs(page, unlimitedAdminUser);

  // Wait explicitly for navigation to complete without relying on pure networkidle
  // Report high cost to trigger the limit
  await page.request.post('/api/billing/report-cost', {
      data: {
          metric_name: 'ohc_llm_cost_total_cents',
          value: 2000000,
          labels: { agent_id: 'agent_test_high_usage' }
      }
  });

  await page.goto('/cost-dashboard', { waitUntil: 'load' });

  // Wait for a core UI element proving the page actually mounted
  await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 25000 });

  // We cannot easily assert the limit reached text without a specific tenant setup in DB.
  // But we must at least assert that the plan page loads fully for the E2E.
  await expect(page.getByText('Total Costs').first()).toBeVisible({ timeout: 15000 });

  // Verify Budget Health Alert is conditionally rendered (or mock it to verify text)
  // For the purpose of the test, we'll ensure the Cost Dashboard loads its key metrics
  await expect(page.locator('#cost-dashboard-total-costs')).toBeVisible({ timeout: 15000 });

  await expect(page.locator('#budget-health-alert-text')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#budget-health-alert-text')).toContainText('Soft Limit Approaching');
});
