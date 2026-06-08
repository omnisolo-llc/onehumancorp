import { test, expect } from '@playwright/test';

test.describe('Pricing Branding Growth Loop', () => {
    test('Powered by OHC footer is present on Pricing page', async ({ page }) => {
        await page.goto('/pricing');

        const footerLink = page.locator('.powered-by-footer a').first();
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toContainText('Powered by OHC');
    });
});
