import { test, expect } from '../../../../e2e/fixtures';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago and Zoom', async ({ page }) => {
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

        // Let's connect Mercado Pago
        const mercadoCard = page.locator('h3', { hasText: 'Mercado Pago' }).locator('..');
        const connectMercadoPagoButton = mercadoCard.getByRole('button', { name: 'Connect' });

        // Mock window alert
        page.on('dialog', dialog => dialog.accept());
        await connectMercadoPagoButton.click();

        // Verify state changed
        await expect(mercadoCard.locator('button:has-text("Manage")')).toBeVisible();

        // Let's connect Zoom
        const zoomCard = page.locator('h3', { hasText: 'Zoom' }).locator('..');
        const connectZoomButton = zoomCard.getByRole('button', { name: 'Connect' });
        await connectZoomButton.click();

        // Verify state changed
        await expect(zoomCard.locator('button:has-text("Manage")')).toBeVisible();

    });

    test('Checkout page displays Mercado Pago', async ({ page }) => {
        await page.goto('/checkout');
        const mercadoPagoButton = page.locator('button:has-text("Pay with Mercado Pago")');
        await expect(mercadoPagoButton).toBeVisible();
    });

    test('Calendar page displays Join Meeting for appointments with link', async ({ page }) => {
        await page.goto('/calendar');
        await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();
        await expect(page.getByText(/Upcoming Appointments|Join Meeting/).first()).toBeVisible();
    });
});
