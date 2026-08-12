import { test, expect } from './fixtures';

test.describe('Pricing Branding Growth Loop', () => {
    test('Powered by OmniSolo footer is present on Pricing page', async ({ page }) => {
        await page.goto('/pricing');

        const footerLink = page.locator('a:has-text("⚡ Powered by OmniSolo")').first();
        await expect(footerLink).toBeVisible();
        await expect(page.locator('text=⚡ Powered by OmniSolo').first()).toBeVisible();
    });
});
