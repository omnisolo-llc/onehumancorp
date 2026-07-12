import { test, expect } from '../../../../e2e/fixtures';

test.describe('Branding Growth Loop', () => {
    test('Powered by OHC footer is present and links correctly', async ({ page }) => {
        await page.goto('/storefront-builder');
        await page.evaluate(() => localStorage.setItem('ohc_builder_status', 'draft'));
        await page.reload();

        const footerLink = page.locator('.powered-by-footer a').first();
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toContainText('Powered by');
        await expect(footerLink).toContainText('OHC');
    });

    test('Website Builder also shows Powered by OHC footer', async ({ page }) => {
        await page.goto('/website-builder');
        await page.evaluate(() => {
            const state = JSON.parse(localStorage.getItem('website-builder-storage') || '{"state":{}}');
            state.state.status = 'draft';
            localStorage.setItem('website-builder-storage', JSON.stringify(state));
        });
        await page.reload();

        const footerLink = page.locator('.powered-by-footer a').first();
        await expect(footerLink).toBeVisible({ timeout: 15000 });
    });
});
