import { test, expect } from '@playwright/test';
test.describe('Viral Share Cards Generator E2E', () => {
    test('Should allow user to preview and attempt to remove branding (soft paywall)', async ({ page }) => {
        const fs = require('fs');
        const path = require('path');
        const filePath = path.resolve('src/ui/tauri/src/ui/share-cards.html');
        await page.goto(`file://${filePath}`);

        await expect(page.locator('h1')).toHaveText('Viral Share Cards Generator');

        await page.fill('#storeName', 'My Awesome Bakery');
        await page.fill('#tagline', 'Delicious treats!');
        await page.selectOption('#theme', 'dark');

        await expect(page.locator('#previewTitle')).toHaveText('My Awesome Bakery');
        await expect(page.locator('#previewTagline')).toHaveText('Delicious treats!');
        await expect(page.locator('#previewCard')).toHaveClass(/dark/);

        // Click instead of check, because the JS event handler unchecks it immediately
        await page.locator('#removeBranding').click({ force: true });

        await expect(page.locator('#paywallModal')).toBeVisible();

        await page.click('#closePaywall');
        await expect(page.locator('#paywallModal')).not.toBeVisible();

        const isChecked = await page.isChecked('#removeBranding');
        expect(isChecked).toBe(false);
    });
});
