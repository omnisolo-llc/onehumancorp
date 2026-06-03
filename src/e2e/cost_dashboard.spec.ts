import { test, expect } from './fixtures';

test.describe('Cost Dashboard E2E', () => {
  test('should display Business Advisory Dashboard title and navigation', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Verify header and back button
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Back to My Plan' })).toBeVisible();
  });

  test('should display the Advisory Summary section', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Verify Advisory Summary content
    await expect(page.getByRole('heading', { name: 'Advisory Summary' })).toBeVisible();
    await expect(page.locator('text=Here\'s what happened this week and what you should do next:')).toBeVisible();
    await expect(page.locator('text=Recommendation: Consider running a seasonal promotion to capitalize on the recent influx of visitors.')).toBeVisible();
  });

  test('should display Cost Transparency overview', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Verify transparency overview headings
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();

    // Verify metrics exist
    await expect(page.getByRole('heading', { name: 'Total Costs' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Total Revenue' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
  });

  test('should correctly render the Cost Breakdown section details', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Verify breakdown heading
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();

    // Verify specific components exist
    await expect(page.locator('text=LLM Usage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();

    await expect(page.locator('text=Storage').nth(0)).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();

    await expect(page.locator('text=Payment Fees')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
  });

  test('should display new network and bandwidth properties safely without errors', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Make sure we wait for it to be rendered successfully without crashing
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();

    // Ensure our newly added state fallback properties show up
    await expect(page.locator('text=Network & Bandwidth')).toBeVisible();
    await expect(page.locator('#cost-dashboard-network')).toBeVisible();

    await expect(page.locator('text=Bandwidth Savings')).toBeVisible();
    await expect(page.locator('#cost-dashboard-bandwidth-savings')).toBeVisible();
  });
});
