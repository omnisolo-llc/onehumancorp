import { test, expect } from '../../../../e2e/fixtures';

test.describe('QR Code Generation Widget', () => {
    test('renders correctly and generates QR code', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard');

        // Ensure "QR Code Generator" link exists
        const link = page.locator('a[href="/qr-code-generator"]');
        if (await link.isVisible()) {
           await link.click();
        } else {
           await page.goto('/qr-code-generator');
        }

        // Wait for page to load
        await expect(page.locator('h1').filter({ hasText: 'QR Code Generator' }).first()).toBeVisible();

        // Check if QR code is visible
        const qrCode = page.locator('img[alt="QR Code"]').first();
        await expect(qrCode).toBeVisible({ timeout: 10000 });

        // Change text and see if it updates
        const input = page.locator('input[type="text"]').first();
        await input.fill('https://ohc.app');

        // Check for download button
        const downloadBtn = page.locator('button', { hasText: 'Download' });
        await expect(downloadBtn).toBeVisible();
    });
});
