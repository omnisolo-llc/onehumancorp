import { test, expect } from './fixtures';
import path from 'path';

test.describe('Receipt Scanner CUJ', () => {
    test('User can upload a receipt and provide amount and vendor', async ({ page }) => {
        // Log in first to get an authenticated session
        await page.goto('/login');
        await page.getByLabel('Email or username').fill('test@example.com');
        await page.getByLabel('Password').fill('password123');
        await page.getByLabel(/Organization/).fill('e2e-tenant');
        await Promise.all([
          page.waitForURL('**/dashboard'),
          page.getByRole('button', { name: 'Log in' }).click(),
        ]);

        // Go to dashboard receipt page
        await page.goto('/dashboard/receipt');

        // Check UI styling/layout
        const container = page.locator('.glassmorphism');
        await expect(container).toBeVisible();

        // Check if amount and vendor inputs are empty (no mock data)
        const amountInput = page.getByTestId('receipt-amount-input');
        const vendorInput = page.getByTestId('receipt-vendor-input');
        const submitBtn = page.getByTestId('submit-receipt-btn');

        await expect(amountInput).toHaveValue('');
        await expect(vendorInput).toHaveValue('');

        // Ensure the button is disabled initially
        await expect(submitBtn).toBeDisabled();

        // Fill data
        await amountInput.fill('150.75');
        await vendorInput.fill('Office Supplies Co');

        // Create a dummy file to upload
        const fileInput = page.getByTestId('receipt-file-input');

        await fileInput.setInputFiles({
            name: 'receipt.jpg',
            mimeType: 'image/jpeg',
            buffer: Buffer.from('image-content-bytes')
        });

        // Wait for UI to update
        await expect(page.getByText('Selected: receipt.jpg')).toBeVisible();

        // Check button is now enabled
        await expect(submitBtn).toBeEnabled();

        // Form submission happens without mocking.
        await submitBtn.click();

        // Toast message should appear indicating the process is completed by the backend.
        const toast = page.getByTestId('receipt-toast');
        await expect(toast).toBeVisible({ timeout: 10_000 });
        await expect(toast).toContainText("Done. Marked as");
    });
});
