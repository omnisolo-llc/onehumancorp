import { test, expect } from '@playwright/test';

test.describe('Snap Receipt E2E Test', () => {
  test('User can fill in receipt details without mock data and upload', async ({ page }) => {
    // 1. Visit the receipt page
    await page.goto('/dashboard/receipt');

    // 2. Verify there is no mock data
    const amountInput = page.getByTestId('receipt-amount-input');
    await expect(amountInput).toHaveValue('');

    const vendorInput = page.getByTestId('receipt-vendor-input');
    await expect(vendorInput).toHaveValue('');

    // 3. Select a real file instead of a mocked buffer
    const fileInput = page.getByTestId('receipt-file-input');
    // Using a real image from the repository
    await fileInput.setInputFiles('src/ui/next/public/onboarding-step1.png');

    // 4. Fill in values manually
    await amountInput.fill('150.75');
    await vendorInput.fill('Best Buy');

    // 5. Submit form
    const submitBtn = page.getByTestId('submit-receipt-btn');
    await expect(submitBtn).toBeEnabled();
    await submitBtn.click();

    // 6. Assert success - The real backend processes the receipt and returns a category.
    const toast = page.getByTestId('receipt-toast');
    await expect(toast).toBeVisible({ timeout: 10000 });
    // Since the actual category returned by the backend depends on its logic, we just check that it indicates success.
    await expect(toast).toContainText("Done. Marked as");
    await expect(toast).toContainText("150.75");
    await expect(toast).toContainText("Best Buy");
  });
});
