import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    await page.getByRole('link', { name: 'Account & Billing' }).click();

    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');

    await page.getByRole('button', { name: 'View Detailed Costs' }).click();

    await expect(page.getByRole('heading', { name: 'Costs' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
  });
});
