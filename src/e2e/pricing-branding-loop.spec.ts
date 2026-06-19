import { test, expect } from './fixtures';

test.describe('Pricing Branding Growth Loop', () => {
    test('Powered by OHC footer is present on Pricing page', async ({ page }) => {
        await page.goto('/pricing');

        const footerLink = page.locator('a:has-text("⚡ Powered by OHC")').first();
        await expect(footerLink).toBeVisible();
        await expect(page.locator('text=⚡ Powered by OHC').first()).toBeVisible();
    });
});
