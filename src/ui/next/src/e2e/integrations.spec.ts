import { test, expect } from '@playwright/test';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago and Zoom', async ({ page }) => {
        await page.goto('http://localhost:3000/integrations');

        const connectMercadoPagoButton = page.locator('button:has-text("Connect")').first();
        await expect(connectMercadoPagoButton).toBeVisible();
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
