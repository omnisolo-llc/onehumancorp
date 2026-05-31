import { test, expect } from './fixtures';

test.describe('Cost Dashboard Data Verification', () => {
  test('should display cost transparency info properly', async ({ page }) => {
    // Navigate to cost dashboard
    await page.goto('/cost-dashboard');

    // Verify dashboard title
    await expect(page.locator('h1').filter({ hasText: 'Business Advisory Dashboard' }).or(page.getByRole('heading', { name: 'Cost Transparency Dashboard' }).first())).toBeVisible();

    // Verify Cost Transparency section
    await expect(page.locator('h2').filter({ hasText: 'Cost Transparency' })).toBeVisible();

    // Verify Total Costs
    await expect(page.getByText('Total Costs').first()).toBeVisible();

    // Verify Cost Breakdown section
    await expect(page.locator('h2').filter({ hasText: 'Cost Breakdown' })).toBeVisible();
    await expect(page.locator('span').filter({ hasText: 'LLM Usage' })).toBeVisible();
    await expect(page.locator('span').filter({ hasText: 'Storage' })).toBeVisible();
    await expect(page.locator('span').filter({ hasText: 'Payment Fees' })).toBeVisible();
  });
});

// Empty comment to trigger push
