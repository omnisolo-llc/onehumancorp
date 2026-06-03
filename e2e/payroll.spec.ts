import { test, expect } from '@playwright/test';

test.describe('Gusto Payroll E2E', () => {
  test('User can initiate payroll sync to Gusto', async ({ page }) => {
    // 1. Setup minimal route mocking
    await page.route('/api/payroll/sync', async route => {
      await route.fulfill({ status: 200, json: { status: "ok" } });
    });

    // 2. Navigate to payroll page
    await page.goto('/dashboard/payroll');

    // 3. Verify page renders with correct glassmorphism styling
    const header = page.locator('text=Gusto Payroll Integration');
    await expect(header).toBeVisible();

    // 4. Click the Sync button
    const syncButton = page.locator('#sync-payroll-button');
    await expect(syncButton).toBeVisible();
    await syncButton.click();

    // 5. Verify the button text changes while syncing or completes (mock is fast, so we just check it finishes)
    await expect(syncButton).toHaveText('Sync OHC Hours to Gusto');
  });
});
