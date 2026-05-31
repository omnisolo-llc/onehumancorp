import { test, expect } from '@playwright/test';

test.describe('Post-Purchase Referral Growth Loop', () => {
    test('displays referral modal and correct links after successful payment', async ({ page }) => {
        // Go to checkout page directly
        await page.goto('http://localhost:3000/checkout');

        // Mock localStorage to simulate a specific tenant
        await page.evaluate(() => {
            localStorage.setItem('tenant_id', 'test-tenant-123');
            localStorage.setItem('token', 'fake-token-456');
        });

        // Intercept the API call to mock the referral generate endpoint
        await page.route('/api/v1/growth/referrals/generate', async route => {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({
                    referral_link: 'ohc://join?ref=test-tenant-123-code'
                })
            });
        });

        // Click the Pay Now button
        const payNowButton = page.getByRole('button', { name: 'Pay Now' });
        await expect(payNowButton).toBeVisible();
        await payNowButton.click();

        // The modal should appear
        const modalHeading = page.locator('h2', { hasText: 'Payment Successful!' });
        await expect(modalHeading).toBeVisible();

        // The referral link input should contain the correct link
        const linkInput = page.locator('input[readonly]');
        await expect(linkInput).toHaveValue('ohc://join?ref=test-tenant-123-code');

        // The WhatsApp and X share links should be correct
        const whatsappLink = page.locator('a', { hasText: 'WhatsApp' });
        await expect(whatsappLink).toHaveAttribute('href', /ohc\.\/\/join\?ref=test-tenant-123-code/);

        const xLink = page.locator('a', { hasText: 'X (Twitter)' });
        await expect(xLink).toHaveAttribute('href', /ohc\.\/\/join\?ref=test-tenant-123-code/);
    });
});
