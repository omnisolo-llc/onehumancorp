import { test, expect } from '../../../../e2e/fixtures';

test.describe('Integrations Loop', () => {
    test('Integrations require verified providers before showing connected state', async ({ page }) => {
        await page.goto('/integrations');

        // Verify all 10 integrations exist with their respective names and descriptions
        await expect(page.locator('h3:has-text("Ayrshare")')).toBeVisible();
        await expect(page.locator('h3:has-text("Cal.com")')).toBeVisible();
        await expect(page.locator('h3:has-text("MailerLite")')).toBeVisible();
        await expect(page.locator('h3:has-text("Mercado Pago")')).toBeVisible();
        await expect(page.locator('h3:has-text("Shippo")')).toBeVisible();
        await expect(page.locator('h3:has-text("Twilio Conversations")')).toBeVisible();
        await expect(page.locator('h3:has-text("Whereby")')).toBeVisible();
        await expect(page.locator('h3:has-text("Resend")')).toBeVisible();
        await expect(page.locator('h3:has-text("Meta Graph API")')).toBeVisible();
        await expect(page.locator('h3:has-text("Front")')).toBeVisible();
        await expect(page.locator('h3:has-text("Zoom")')).toBeVisible();

        const mercadoCard = page.locator('h3', { hasText: 'Mercado Pago' }).locator('..');
        const connectMercadoPagoButton = mercadoCard.getByRole('button', { name: 'Connect' });
        await connectMercadoPagoButton.click();
        await expect(page.getByRole('status')).toHaveText(
          'Mercado Pago connection is unavailable until secure provider verification is configured.',
        );
        await expect(mercadoCard.getByRole('button', { name: 'Manage' })).toHaveCount(0);

        const zoomCard = page.locator('h3', { hasText: 'Zoom' }).locator('..');
        const connectZoomButton = zoomCard.getByRole('button', { name: 'Connect' });
        await connectZoomButton.click();
        await expect(page.getByRole('status')).toHaveText(
          'Zoom connection is unavailable until secure provider verification is configured.',
        );
        await expect(zoomCard.getByRole('button', { name: 'Manage' })).toHaveCount(0);
    });

    test('Checkout verifies catalog product details before offering payment', async ({ page }) => {
        await page.goto('/checkout?product_id=e2e-product-cake&quantity=2');
        await expect(page.getByRole('heading', { name: 'Vegan Celebration Cake' })).toBeVisible();
        await expect(page.getByText('$79.98')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Pay', exact: true })).toBeVisible();
    });

    test('Calendar displays database-backed product bookings', async ({ page }) => {
        await page.goto('/calendar');
        await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();
        await expect(page.getByRole('heading', { name: 'Cake Decorating Class' })).toBeVisible();
        await expect(page.getByText('Failed to load appointments. Please try again later.')).toHaveCount(0);
    });
});
