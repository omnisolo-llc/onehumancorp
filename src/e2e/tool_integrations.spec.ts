import { test, expect } from '@playwright/test';

test.describe('External Tool Integrations CUJ', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/login');
        await page.fill('input[placeholder*="Email"]', 'test@example.com');
        await page.fill('input[type="password"]', 'password');
        await page.click('button[type="submit"]');
        await page.waitForTimeout(500);
    });

    test('should fully integrate Chatwoot unified inbox routing', async ({ page }) => {
        await page.goto('/settings/integrations');
        await page.click('button:has-text("Chatwoot")');
        await expect(page.locator('text=Unified Inbox')).toBeVisible();
        await page.click('button:has-text("Connect Chatwoot")');
        await expect(page.locator('text=Connected')).toBeVisible();

        await page.goto('/inbox');
        await expect(page.locator('text=External Messages')).toBeVisible();
    });

    test('should process Cal.com storefront booking adjustments', async ({ page }) => {
        await page.goto('/settings/integrations');
        await page.click('button:has-text("Cal.com")');
        await page.click('button:has-text("Enable Booking")');

        await page.goto('/storefront');
        await expect(page.locator('.booking-widget')).toBeVisible();
    });

    test('should complete Mercado Pago checkout flow integration', async ({ page }) => {
        await page.goto('/settings/integrations');
        await page.click('button:has-text("Mercado Pago")');
        await page.click('button:has-text("Connect Mercado Pago")');

        await page.goto('/checkout');
        await expect(page.locator('text=Pay with Mercado Pago')).toBeVisible();
    });

    test('should generate Shippo labels on order fulfillment UI', async ({ page }) => {
        await page.goto('/orders/123');
        await expect(page.locator('button:has-text("Create Shipping Label")')).toBeVisible();
        await page.click('button:has-text("Create Shipping Label")');
        await expect(page.locator('text=Label Created')).toBeVisible();
    });

    test('should draft Resend email campaigns via marketing tab', async ({ page }) => {
        await page.goto('/marketing/campaigns');
        await page.click('button:has-text("New Email Campaign")');
        await page.fill('input[placeholder*="Subject"]', 'Summer Sale');
        await page.click('button:has-text("Send with Resend")');
        await expect(page.locator('text=Campaign Queued')).toBeVisible();
    });
});
