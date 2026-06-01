import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should navigate to the cost dashboard and view cost transparency data', async ({ page }) => {
    // Navigate to the Dashboard (starts on home)
    await page.goto('/cost-dashboard');

    // Verify Cost Dashboard components
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' }).first()).toBeVisible();

    // Verify the Billing Period is shown
    await expect(page.getByText('Period:')).toBeVisible();

    // Verify specific costs are listed
    await expect(page.getByText('LLM Inference Cost').first()).toBeVisible();
    await expect(page.getByText('Storage & CDN').first()).toBeVisible();
    await expect(page.getByText('Payment Processor Fees').first()).toBeVisible();
    await expect(page.getByText('Total Costs').first()).toBeVisible();
  });
});
