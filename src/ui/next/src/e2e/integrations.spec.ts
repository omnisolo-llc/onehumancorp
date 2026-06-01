import { test, expect } from '@playwright/test';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago and Zoom', async ({ page }) => {
        await page.goto('http://localhost:3000/integrations');

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
        await expect(page.locator('h3:has-text("Zoom")')).toBeVisible();

        // Let's connect Mercado Pago
        const mercadoCard = page.locator('div').filter({ hasText: 'Mercado Pago' }).first();
        const connectMercadoPagoButton = mercadoCard.locator('button:has-text("Connect")');

        // Mock window alert
        page.on('dialog', dialog => dialog.accept());
        await connectMercadoPagoButton.click();

        // Verify state changed
        await expect(mercadoCard.locator('button:has-text("Manage")')).toBeVisible();

        // Let's connect Zoom
        const zoomCard = page.locator('div').filter({ hasText: 'ZoomAutomated' }).first();
        const connectZoomButton = zoomCard.locator('button:has-text("Connect")');
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

    test('Integrations loop connects Twilio WhatsApp', async ({ page }) => {
        await page.goto('http://localhost:3000/integrations');

        const twilioCard = page.locator('div').filter({ hasText: 'Twilio Conversations' }).first();
        const connectTwilioButton = twilioCard.locator('button:has-text("Connect")');
        await connectTwilioButton.click();

        // Modal should appear
        await expect(page.locator('h2:has-text("Connect Twilio Conversations")')).toBeVisible();

        // Verify WhatsApp option is visible
        await expect(page.locator('span:has-text("WhatsApp Business API")')).toBeVisible();

        // Save & Connect
        await page.locator('button:has-text("Save & Connect")').click();

        // Should redirect to inbox
        await page.waitForURL('**/inbox');
        await expect(page.locator('h1:has-text("Customer Inbox")')).toBeVisible();
    });
});
