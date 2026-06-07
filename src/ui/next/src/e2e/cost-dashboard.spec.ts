import { test, expect } from '@playwright/test';

// NOTE: This test requires a docker-sandbox fix to run properly in CI
// due to pgvector pull permissions in the Bazel test sandbox environment.
test.describe('Cost Dashboard Loop', () => {
  test('Cost dashboard loads and displays data', async ({ page }) => {
    // Navigate to the dashboard page
    await page.goto('/cost-dashboard');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });

    // Check that the My Plan is present
    await expect(page.locator('h2', { hasText: 'My Plan' })).toBeVisible();

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
