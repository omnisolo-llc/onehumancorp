import { test, expect } from '@playwright/test';

test.describe('Storefront QR Code Generator', () => {
    test('should generate a QR code for the storefront link and display it correctly', async ({ page }) => {
        // Navigate to the share cards page
        await page.goto('/share-cards');

        // Check if the page title exists
        await expect(page.getByRole('heading', { name: 'Social Share Cards' })).toBeVisible();

        // Check for the Storefront QR Code component
        const qrCodeContainer = page.getByTestId('qr-code-container');
        await expect(qrCodeContainer).toBeVisible();
        await expect(page.getByRole('heading', { name: 'Storefront QR Code' })).toBeVisible();

        // Check for the actual QR Code SVG
        await expect(page.getByTestId('qr-code-svg')).toBeVisible();

        // Check for the Powered by OHC viral branding
        await expect(page.getByText('Powered by OHC', { exact: true })).toBeVisible();

        // Set a custom store name and check if the QR code text updates
        await page.getByLabel('Store Name').fill('Fatimas Halal Cart');
        await expect(page.getByText('Scan to visit Fatimas Halal Cart')).toBeVisible();
    });
});
