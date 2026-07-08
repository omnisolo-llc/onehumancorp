import { test, expect } from '@playwright/test';

test.describe('Documentation Features Flow', () => {

    test('Help Center page UI loads and structure is visible', async ({ page }) => {
        // Go to the help widget UI
        await page.goto('/help.html');
        await expect(page.locator('h1').filter({ hasText: 'In-App Help Center' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Getting Started' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'My Store' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Payments' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'AI Agents' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Marketing' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Account & Billing' })).toBeVisible();
    });

    test('Changelog UI loads', async ({ page }) => {
        await page.goto('/changelog.html');
        await expect(page.locator('h1').filter({ hasText: 'Release Notes & Changelog' })).toBeVisible();
    });

    test('API Docs UI loads', async ({ page }) => {
        await page.goto('/api-docs.html');
        await expect(page.locator('title').filter({ hasText: 'API Documentation' })).toBeVisible({ timeout: 1000 });
    });

});
