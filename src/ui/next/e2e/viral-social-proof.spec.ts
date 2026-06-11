import { test, expect } from '@playwright/test';

test.describe('Viral Social Proof Nudge', () => {
    test('generator page renders correctly, saves data, and embeds code works with viral footer', async ({ page }) => {
        // 1. Set some initial local storage state to act as a logged-in user
        await page.goto('/dashboard');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-bakery');
        });

        // 2. Go to the Social Proof Nudge page
        await page.goto('/social-proof-nudge');

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Social Proof Nudge' })).toBeVisible();

        // 3. Configure the nudge
        const productNameInput = page.getByPlaceholder('e.g. Signature Coffee Blend');
        await productNameInput.fill('Awesome E2E Cake');

        const locationInput = page.getByPlaceholder('e.g. Someone in London');
        await locationInput.fill('Someone in San Francisco');

        // Check preview
        await expect(page.locator('p', { hasText: 'Someone in San Francisco' })).toBeVisible();
        await expect(page.getByText('Awesome E2E Cake', { exact: true })).toBeVisible();

        // Verify the viral footer exists in the preview
        const publicFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();

        // Check the generated embed code
        const embedCode = await page.locator('#embed-code').textContent();
        expect(embedCode).toContain('data-product="Awesome E2E Cake"');
        expect(embedCode).toContain('data-location="Someone in San Francisco"');
        expect(embedCode).toContain('Powered by OHC');
    });
});
