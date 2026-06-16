import { test, expect } from '@playwright/test';

test.describe('Release Notes & Changelog', () => {
    test('renders Changelog page', async ({ page }) => {
        await page.goto('/changelog');
        await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    });
});
