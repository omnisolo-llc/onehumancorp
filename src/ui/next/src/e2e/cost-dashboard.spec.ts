import { test, expect } from '@playwright/test';

// NOTE: This test requires a docker-sandbox fix to run properly in CI
// due to pgvector pull permissions in the Bazel test sandbox environment.
test.describe('Cost Dashboard Loop', () => {
  test('Cost dashboard loads and displays main components', async ({ page }) => {
    // Navigate to the dashboard page
    await page.goto('http://localhost:3000/cost-dashboard');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 10000 });

    // Check that the Advisory Summary is present
    await expect(page.locator('h2', { hasText: 'Advisory Summary' })).toBeVisible();

    // Check that the Cost Transparency section is present
    await expect(page.locator('h2', { hasText: 'Cost Transparency' })).toBeVisible();

    // Check that Cost Breakdown section is present
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();
  });

  test('Cost dashboard displays total revenue', async ({ page }) => {
    await page.goto('http://localhost:3000/cost-dashboard');
    await expect(page.locator('h2', { hasText: 'Total Revenue' })).toBeVisible({ timeout: 10000 });
    const revenueValue = page.locator('#cost-dashboard-revenue');
    await expect(revenueValue).toBeVisible();
    await expect(revenueValue).toContainText('$');
  });

  test('Cost dashboard displays total costs', async ({ page }) => {
    await page.goto('http://localhost:3000/cost-dashboard');
    await expect(page.locator('h2', { hasText: 'Total Costs' })).toBeVisible({ timeout: 10000 });
    const costValue = page.locator('#cost-dashboard-total');
    await expect(costValue).toBeVisible();
    await expect(costValue).toContainText('$');
  });

  test('Cost dashboard breakdown items have descriptions', async ({ page }) => {
    await page.goto('http://localhost:3000/cost-dashboard');

    // Check for individual breakdown items
    await expect(page.locator('span', { hasText: 'LLM Usage' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('p', { hasText: 'Cost of AI agent actions and interactions.' })).toBeVisible();

    await expect(page.locator('span', { hasText: 'Storage' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Cost of cloud storage and file hosting.' })).toBeVisible();

    await expect(page.locator('span', { hasText: 'Payment Fees' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Stripe transaction fees on processed revenue.' })).toBeVisible();
  });

  test('Cost dashboard navigation to My Plan page', async ({ page }) => {
    await page.goto('http://localhost:3000/cost-dashboard');
    // Check navigation works
    const backBtn = page.locator('button', { hasText: 'Back to My Plan' });
    await expect(backBtn).toBeVisible({ timeout: 10000 });
    await backBtn.click();
    await expect(page).toHaveURL('http://localhost:3000/plan');
  });
});
