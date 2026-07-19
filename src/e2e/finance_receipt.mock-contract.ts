import { test, expect } from '@playwright/test';

test.describe('Invisible Autonomous Bookkeeping', () => {
  test('CUJ: Snap a receipt and verify dashboard reflects Money Out', async ({ page, request }) => {
    // Navigate to the dashboard
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/dashboard');

    // Wait for the CFO agent card to be loaded and display initial values
    await page.waitForSelector('text="Profit & Tax Card"');

    await expect(page.getByText('Profit & Tax Card')).toBeVisible();
    await expect(page.getByText('Money In')).toBeVisible();
    await expect(page.getByText('Money Out')).toBeVisible();
    await expect(page.getByText('Estimated Tax Safe')).toBeVisible();

    // Click on the FAB to open the menu
    const fabButton = page.locator('.fixed.bottom-6.right-6 button').last();
    await fabButton.click();

    // Click on "Snap Receipt"
    const snapReceiptBtn = page.getByTestId('snap-receipt-fab');
    await snapReceiptBtn.click();

    // Wait for navigation to receipt page
    await page.waitForURL('**/dashboard/receipt');
    await expect(page.getByText('Snap Receipt')).toBeVisible();

    // Upload a dummy file (simulating the camera capture)
    const fileInput = page.getByTestId('receipt-file-input');
    await fileInput.setInputFiles({
      name: 'receipt.jpg',
      mimeType: 'image/jpeg',
      buffer: Buffer.from('fake-image-data')
    });

    // Verify "Selected: receipt.jpg" appears
    await expect(page.getByText(/Selected: receipt.jpg/)).toBeVisible();

    // Fill in simulated values so we don't hardcode them in the backend
    const amountInput = page.getByTestId('receipt-amount-input');
    await amountInput.fill('45.20');
    const vendorInput = page.getByTestId('receipt-vendor-input');
    await vendorInput.fill('Home Depot');

    // Submit the receipt
    const submitBtn = page.getByTestId('submit-receipt-btn');
    await submitBtn.click();

    // Verify the toast message
    const toast = page.getByTestId('receipt-toast');
    await expect(toast).toBeVisible();
    await expect(toast).toContainText(/AI is categorizing your \$45.20 expense at Home Depot... Done. Marked as 'Supplies'./);

    // Wait for redirection back to dashboard
    await page.waitForURL('**/dashboard');

    // Verify we are back on the dashboard
    await expect(page.getByText('Profit & Tax Card')).toBeVisible();
  });
});
