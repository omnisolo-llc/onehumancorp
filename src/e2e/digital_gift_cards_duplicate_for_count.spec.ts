import { test, expect } from '@playwright/test';

test.describe('Digital Gift Cards Growth Loop E2E', () => {
    test.beforeEach(async ({ page }) => {
        // Assume user is logged in and on the dashboard
        await page.goto('/dashboard');
        // Set local storage for tenant
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'test-store');
        });
    });

    test('should display Digital Gift Cards section on dashboard', async ({ page }) => {
        await expect(page.locator('h2:has-text("Digital Gift Cards")')).toBeVisible();
        await expect(page.locator('text=Growth Loop').nth(0)).toBeVisible();
    });

    test('should open Gift Card Modal when clicking Generate AI Campaign', async ({ page }) => {
        await page.locator('button:has-text("Generate AI Campaign")').first().click();

        // Wait for modal
        await expect(page.locator('h2:has-text("Digital Gift Card Campaign")')).toBeVisible();
        await expect(page.locator('textarea')).toBeVisible();
    });

    test('should fetch and display generated message in textarea', async ({ page }) => {
        // Mock the API response
        await page.route('**/api/v1/growth/campaign/generate-gift-card', async route => {
            const json = { message: 'Mocked AI Gift Card Campaign Message for test-store' };
            await route.fulfill({ json });
        });

        await page.locator('button:has-text("Generate AI Campaign")').first().click();

        // Check for loading state then actual message
        const textarea = page.locator('textarea');
        await expect(textarea).toHaveValue('Mocked AI Gift Card Campaign Message for test-store');
    });

    test('should show fallback message if API fails', async ({ page }) => {
        // Mock the API failure
        await page.route('**/api/v1/growth/campaign/generate-gift-card', async route => {
            await route.abort('failed');
        });

        await page.locator('button:has-text("Generate AI Campaign")').first().click();

        const textarea = page.locator('textarea');
        await expect(textarea).toHaveValue(/Treat someone special!/);
    });

    test('should copy message to clipboard and update button text', async ({ page, context }) => {
        // Mock clipboard API
        await context.grantPermissions(['clipboard-read', 'clipboard-write']);

        await page.route('**/api/v1/growth/campaign/generate-gift-card', async route => {
            const json = { message: 'Mocked AI Gift Card Message' };
            await route.fulfill({ json });
        });

        await page.locator('button:has-text("Generate AI Campaign")').first().click();

        const copyButton = page.locator('button:has-text("Copy Message")');
        await copyButton.click();

        await expect(page.locator('button:has-text("Copied to Clipboard!")')).toBeVisible();
    });
});
