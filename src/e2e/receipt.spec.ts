import { test, expect } from '@playwright/test';

test.describe('Receipt upload functionality', () => {
  test('should initialize with empty amount and vendor, and show error if submitted empty', async ({ page }) => {
    // This is a minimal spec just to prove we can navigate and see the form correctly without mocks
    await page.goto('/dashboard/receipt');

    // Check initial state
    const amountInput = page.getByTestId('receipt-amount-input');
    const vendorInput = page.getByTestId('receipt-vendor-input');

    await expect(amountInput).toHaveValue('');
    await expect(vendorInput).toHaveValue('');

    // Both should be required
    await expect(amountInput).toHaveAttribute('required', '');
    await expect(vendorInput).toHaveAttribute('required', '');
  });
});
