import { test, expect } from './fixtures';

test.describe('Exit Intent Growth Loop', () => {
    test('renders Powered by OHC footer in the embed code', async ({ page }) => {
        // Go to the exit intent generator
        await page.goto('/exit-intent-generator');

        // Check the page header to make sure it loaded
        await expect(page.locator('h2', { hasText: 'Configure Exit Intent Pop-up' })).toBeVisible();

        // Check the preview
        await expect(page.locator('h3', { hasText: 'Wait! Don\'t leave yet.' })).toBeVisible();
        await expect(page.locator('span', { hasText: '⚡ Powered by OHC' })).toBeVisible();

        // Ensure the referral growth loop is intact in the generated embed code
        const embedCode = await page.locator('pre').innerText();
        expect(embedCode).toContain('id="ohc-exit-intent"');
        expect(embedCode).toContain('⚡ Powered by OHC');

        // Ensure "Powered by OHC" is disabled unless Pro
        const checkbox = page.locator('input[type="checkbox"]');
        await expect(checkbox).toBeDisabled();
    });
});
