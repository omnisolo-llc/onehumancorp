import { test, expect } from '@playwright/test';

test.describe('Integrations Loop', () => {
    test('Integrations loop connects Mercado Pago, Zoom, Meta, Twilio, Cal.com, Resend, and Shippo', async ({ page }) => {
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

        // Helper function to mock alert and connect
        const connectIntegration = async (name: string, alertTextSubstring?: string) => {
            const card = page.locator('div').filter({ hasText: new RegExp('^' + name) }).first();
            const connectButton = card.locator('button:has-text("Connect")');
            if (alertTextSubstring) {
                page.on('dialog', dialog => {
                    if (dialog.message().includes(alertTextSubstring)) {
                        dialog.accept();
                    } else {
                        dialog.dismiss();
                    }
                });
            } else {
                page.once('dialog', dialog => dialog.accept());
            }
            await connectButton.click();
            await expect(card.locator('button:has-text("Manage")')).toBeVisible();
        };

        await connectIntegration('Mercado Pago');
        await connectIntegration('Zoom');
        await connectIntegration('Meta Graph API');
        await connectIntegration('Cal.com');
        await connectIntegration('Resend');

        // Twilio has a modal
        const twilioCard = page.locator('div').filter({ hasText: /^Twilio Conversations/ }).first();
        const connectTwilioButton = twilioCard.locator('button:has-text("Connect")');
        await connectTwilioButton.click();
        const twilioModal = page.locator('h2:has-text("Connect Twilio Conversations")');
        await expect(twilioModal).toBeVisible();
        await page.locator('button:has-text("Save & Connect")').click();

        // it redirects to inbox
        await expect(page).toHaveURL(/.*\/inbox/);
        await page.goto('http://localhost:3000/integrations');
        await expect(twilioCard.locator('button:has-text("Manage")')).toBeVisible();
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
