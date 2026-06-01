import { test, expect } from '@playwright/test';

test.describe('Hardware Leasing Engine', () => {
  test('Carlos can approve an equipment lease with 1-tap', async ({ page }) => {
    // Navigate to the equipment leasing page (simulate receiving a notification link)
    // The E2E tests should start from the home page.
    await page.goto('/dashboard');

    // Create a mock link or directly go to the equipment lease for a specific job
    await page.goto('/equipment-lease/concrete-pouring-friday');

    // Verify the UI elements are loaded correctly
    await expect(page.locator('h1')).toHaveText('Equipment Leasing Engine');
    await expect(page.locator('h2').first()).toHaveText('Suggested Rental');
    await expect(page.locator('text=You need a Cement Mixer for Friday\'s "Concrete Pouring" job.')).toBeVisible();
    await expect(page.locator('text=123 Main St (Friday, 7:00 AM)')).toBeVisible();
    await expect(page.locator('text=Friday, 6:00 PM')).toBeVisible();
    await expect(page.locator('text=$150 / day')).toBeVisible();

    // Intercept the API route for the unified ledger
    await page.route('/api/v1/ledger/lease', async route => {
      await route.fulfill({ status: 200, json: { status: 'success', deposit_secured: 50, job_id: 'concrete-pouring-friday' } });
    });

    // Click the 1-Tap Approve button
    await page.click('button:has-text("Approve 1-Tap Lease")');

    // Verify the success state
    await expect(page.locator('h2').first()).toHaveText('Lease Secured');
    await expect(page.locator('text=The Cement Mixer deposit has been processed. The $150 expense will be automatically deducted from the final job payout.')).toBeVisible();
  });
});
