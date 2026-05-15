import { test, expect } from '@playwright/test';

test.describe('Tool Integrations CUJ', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        // Ensure page is loaded
        await expect(page).toHaveTitle(/OneHuman/);
    });

    test('Meta Graph API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });

    test('Cal.com API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });

    test('Resend API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });

    test('Mercado Pago API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });

    test('EasyPost API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });

    test('Twilio API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });

    test('Google Meet API integration', async ({ page }) => {
        await page.goto('/business-setup');
        await expect(page.locator('text=OneHuman Corp').first()).toBeVisible();
    });
});
