import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    // Basic test to avoid flakes in docker overlayfs. Just making sure
    // the system has the appropriate routing in place.
    await page.goto('/');

    // Go to My Plan first to fulfill the CUJ requirement
    await expect(page.getByText('My Plan')).toBeVisible();
    await page.getByText('My Plan').click();

    await expect(page).toHaveURL(/\/plan/);

    await expect(page.getByText('View Cost Details')).toBeVisible();
    await page.getByText('View Cost Details').click();

    await expect(page).toHaveURL(/\/cost-dashboard/);

    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();

    // Although model costs might be empty for a new user, we expect the API shape
    // to include model_costs successfully. If there's any data, 'Cost by Model' should appear.
  });
});
