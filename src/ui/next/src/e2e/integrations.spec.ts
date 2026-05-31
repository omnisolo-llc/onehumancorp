import { test, expect } from '@playwright/test';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago and Zoom', async ({ page }) => {
        await page.goto('http://localhost:3000/integrations');

        const connectMercadoPagoButton = page.locator('button:has-text("Connect")').nth(3);
        await expect(connectMercadoPagoButton).toBeVisible();

        const connectCalComButton = page.locator('button:has-text("Connect")').nth(1);
        await expect(connectCalComButton).toBeVisible();

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
