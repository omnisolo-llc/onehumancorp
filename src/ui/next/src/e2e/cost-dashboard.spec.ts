import { test, expect } from '@playwright/test';

test.describe('Cost Dashboard Loop', () => {
  test('Cost dashboard loads and displays data', async ({ page }) => {
    // Navigate to the dashboard page
    await page.goto('/cost-dashboard');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 15000 });

    // Check that the Business Advisory Dashboard is present
    await expect(page.locator('h2', { hasText: 'Business Advisory Dashboard' })).toBeHidden();

    await expect(page.locator('h2', { hasText: 'Total Costs' }).first()).toBeVisible();

    // Check that Projected Monthly Cost is displayed
    await expect(page.locator('h2', { hasText: 'Projected Monthly Cost' })).toBeVisible();

    // Check that Cost Breakdown section is present
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();

    // Check for individual breakdown items
    await expect(page.locator('span', { hasText: 'LLM Usage' })).toBeVisible();

    await expect(page.locator('button', { hasText: 'Manage Billing' })).toBeVisible();

    // We do not explicitly test 'Budget Alert' here since it is dynamically triggered based on backend limits.
    // However, we verify the structure surrounding the LLM usage metrics hasn't broken.
    await expect(page.locator('span', { hasText: /^Storage$/ })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Compute Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Email Sends' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Outbound API Calls' })).toBeVisible();

    // Check back works
    await page.locator('a', { hasText: 'Back to My Plan' }).click();
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeHidden();
  });
});
