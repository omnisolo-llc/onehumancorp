import { test, expect } from '@playwright/test';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago and Zoom', async ({ page }) => {
        await page.goto('http://localhost:3000/integrations');

        // Verify all 7 integrations exist with their respective names and descriptions
        await expect(page.locator('h3:has-text("Unified Inbox")')).toBeVisible();
        await expect(page.locator('h3:has-text("Autonomous Booking Agent")')).toBeVisible();
        await expect(page.locator('h3:has-text("Shipping Labels")')).toBeVisible();
        await expect(page.locator('h3:has-text("Local Payments")')).toBeVisible();
        await expect(page.locator('h3:has-text("Customer Emails")')).toBeVisible();
        await expect(page.locator('h3:has-text("Online Meetings")')).toBeVisible();
        await expect(page.locator('h3:has-text("Text Notifications")')).toBeVisible();

        // Let's connect Mercado Pago
        const mercadoCard = page.locator('div').filter({ hasText: 'Local Payments' }).first();
        const connectMercadoPagoButton = mercadoCard.locator('button').filter({ hasText: /^Connect|Accept local payments$/i });

        // Mock window alert
        page.on('dialog', dialog => dialog.accept());
        await connectMercadoPagoButton.click();

        // Verify state changed
        await expect(mercadoCard.locator('button:has-text("Manage")')).toBeVisible();

        // Let's connect Zoom
        const zoomCard = page.locator('div').filter({ hasText: 'Online Meetings' }).first();
        const connectZoomButton = zoomCard.locator('button').filter({ hasText: /^Connect|Create my meeting room$/i });
        await connectZoomButton.click();

        // Verify state changed
        await expect(zoomCard.locator('button:has-text("Manage")')).toBeVisible();

    });

    test('Checkout page displays Mercado Pago', async ({ page }) => {
        await page.goto('http://localhost:3000/checkout');
        const mercadoPagoButton = page.locator('button:has-text("Pay with Mercado Pago")');
        await expect(mercadoPagoButton).toBeVisible();
    });

    test('Calendar page displays Join Meeting for appointments with link', async ({ page }) => {
        await page.goto('http://localhost:3000/calendar');
        const joinMeetingButton = page.locator('a:has-text("Join Meeting")');
        await expect(joinMeetingButton).toBeVisible();
    });
});
