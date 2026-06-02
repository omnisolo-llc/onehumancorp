import { test, expect } from '@playwright/test';

test.describe('Checkout Post-Purchase QR Code Share Growth Loop', () => {
    test('renders QR Code with referral link on payment success', async ({ page }) => {
        // Mock the referral api
        await page.route('/api/v1/growth/referrals/generate', async route => {
            const json = { referral_link: 'http://e2e.test.link' };
            await route.fulfill({ json });
        });

        await page.goto('/checkout');

        // Ensure checkout page loaded
        await expect(page.locator('h1', { hasText: 'Checkout' })).toBeVisible();

        // Click Pay Now
        const payButton = page.locator('button', { hasText: 'Pay Now' });
        await payButton.click();

        // Wait for the modal
        const modalHeader = page.locator('h2', { hasText: 'Payment Successful!' });
        await expect(modalHeader).toBeVisible();

        // Verify the QR code container exists
        const qrContainer = page.getByTestId('qr-code-container');
        await expect(qrContainer).toBeVisible();

        // Verify the link is also in the text input
        const linkInput = page.locator('input[readonly]');
        await expect(linkInput).toHaveValue('http://e2e.test.link');
    });
});
