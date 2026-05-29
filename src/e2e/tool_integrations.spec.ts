import { test, expect } from '@playwright/test';

test.describe('Tool Integrations', () => {
    test.beforeEach(async ({ page }) => {
        // Assume user is already logged in for e2e (using standalone HTML for fast verification)
        await page.goto('file:///app/src/server/assets/index.html');
    });

    test('should display ManyChat integration card and respond to click', async ({ page }) => {
        const manyChatCard = page.locator('text=ManyChat');
        await expect(manyChatCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Connecting to ManyChat...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Connect my Instagram and Facebook' }).click();
    });

    test('should display Cal.com integration card and respond to click', async ({ page }) => {
        const calcomCard = page.locator('text=Cal.com');
        await expect(calcomCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Enabling Cal.com...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Enable Booking Agent' }).click();
    });

    test('should display MailerLite integration card and respond to click', async ({ page }) => {
        const mailerLiteCard = page.locator('text=MailerLite');
        await expect(mailerLiteCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Setting up MailerLite...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Start sending emails' }).click();
    });

    test('should display Mercado Pago integration card and respond to click', async ({ page }) => {
        const mercadoPagoCard = page.locator('text=Mercado Pago');
        await expect(mercadoPagoCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Setting up Mercado Pago...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Accept local payments' }).click();
    });

    test('should display Shippo integration card and respond to click', async ({ page }) => {
        const shippoCard = page.locator('text=Shippo');
        await expect(shippoCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Setting up Shippo...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Set up shipping' }).click();
    });

    test('should display Twilio integration card and respond to click', async ({ page }) => {
        const twilioCard = page.locator('text=Twilio');
        await expect(twilioCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Connecting to Twilio...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Enable text messages' }).click();
    });

    test('should display Whereby integration card and respond to click', async ({ page }) => {
        const wherebyCard = page.locator('text=Whereby');
        await expect(wherebyCard).toBeVisible();
        page.once('dialog', dialog => {
            expect(dialog.message()).toBe('Setting up Whereby...');
            dialog.accept().catch(() => {});
        });
        await page.getByRole('button', { name: 'Create my meeting room' }).click();
    });
});
