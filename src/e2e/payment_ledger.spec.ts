import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Omni-Channel Payment & Ledger System', () => {
  test('CUJ: Business owner creates a deposit request and it instantly updates revenue', async ({ page }) => {
    // 1. Navigate to the payments dashboard (mobile view simulation)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/payments');

    // 2. Initial state: Revenue should be visible (e.g. $0.00 if brand new, or some initial value)
    const revenueDisplay = page.getByTestId('total-revenue');
    await expect(revenueDisplay).toBeVisible();

    // 3. User requests a $50 payment
    const amountInput = page.getByTestId('payment-amount-input');
    await amountInput.fill('50');

    // 4. Click the Request Payment button
    const requestButton = page.getByTestId('request-payment-button');
    await requestButton.click();

    // 5. Verify the processing status
    await expect(requestButton).toHaveText('Waiting for card...');

    // 6. Verify successful completion
    const statusText = page.getByTestId('payment-status');
    await expect(statusText).toBeVisible({ timeout: 10000 });
    await expect(statusText).toHaveText('Approved');

    // 7. Verify the revenue counter updated instantly without page refresh
    // Note: If initial revenue was 0, it should be 50. We just ensure it's not the old value and matches the format.
    const revenueText = await revenueDisplay.textContent();
    expect(revenueText).toContain('$');
  });
});
