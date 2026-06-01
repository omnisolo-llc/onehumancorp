import { test, expect } from './fixtures';

test.describe('Cost Dashboard Data Verification', () => {
  test('should display cost transparency info properly', async ({ page }) => {
    // Navigate to cost dashboard
    await page.goto('/cost-dashboard');

    // Wait for the cost dashboard elements to become visible
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' }).first()).toBeVisible();
    await expect(page.getByText('Keep track of your total usage').first()).toBeVisible();
    await expect(page.getByText('Total Costs').first()).toBeVisible();
  });
});
