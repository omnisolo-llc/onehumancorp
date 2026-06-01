import { test, expect } from '@playwright/test';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago and Jitsi Meet', async ({ page }) => {
        await page.goto('http://localhost:3000/integrations');

        // Verify all integrations exist with their respective names and descriptions
        await expect(page.locator('h3:has-text("Ayrshare")')).toBeVisible();
        await expect(page.locator('h3:has-text("Cal.com")')).toBeVisible();
        await expect(page.locator('h3:has-text("Listmonk")')).toBeVisible();
        await expect(page.locator('h3:has-text("Mercado Pago")')).toBeVisible();
        await expect(page.locator('h3:has-text("EasyPost")')).toBeVisible();
        await expect(page.locator('h3:has-text("Twilio")')).toBeVisible();
        await expect(page.locator('h3:has-text("Jitsi Meet")')).toBeVisible();

        // Let's connect Mercado Pago
        const mercadoCard = page.locator('div').filter({ hasText: 'Mercado Pago' }).first();
        const connectMercadoPagoButton = mercadoCard.locator('button:has-text("Connect")');

        // Mock window alert
        page.on('dialog', dialog => dialog.accept());
        await connectMercadoPagoButton.click();

        // Verify state changed
        await expect(mercadoCard.locator('button:has-text("Manage")')).toBeVisible();

        // Let's connect Jitsi Meet
        const jitsiCard = page.locator('div').filter({ hasText: 'Jitsi Meet' }).first();
        const connectJitsiButton = jitsiCard.locator('button:has-text("Connect")');
        await connectJitsiButton.click();

        // Verify state changed
        await expect(jitsiCard.locator('button:has-text("Manage")')).toBeVisible();

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
