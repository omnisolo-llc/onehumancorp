import { test, expect } from './fixtures';

test.describe('Viral Waitlist Generator E2E', () => {
    test('should allow member to customize waitlist and generate embed code with branding', async ({ memberPage }) => {
        test.setTimeout(90000);

        // Navigate to the generator page
        await memberPage.goto('/pre-order-widget');

        // Wait for the page to load
        await expect(memberPage.locator('h1', { hasText: 'Pre-Order Waitlist Engine' })).toBeVisible({ timeout: 15000 });

        // Update product name
        const productInput = memberPage.locator('input[placeholder="e.g. The Vegan Chocolate Cake"]');
        await productInput.fill('Playwright Test Launch');

        // Verify the preview updates
        await expect(memberPage.locator('h2', { hasText: 'Playwright Test Launch' })).toBeVisible();

        // Click generate widget code
        await memberPage.locator('button', { hasText: 'Get Widget Embed Code' }).click();

        // Check the generated embed code modal
        const embedModal = memberPage.locator('h2', { hasText: 'Embed Your Waitlist' });
        await expect(embedModal).toBeVisible();

        const embedCode = await memberPage.locator('div.font-mono').textContent();
        expect(embedCode).toContain('Playwright Test Launch');
    });

    test('should allow member to change theme', async ({ memberPage }) => {
        // Navigate to the generator page
        await memberPage.goto('/pre-order-widget');

        // Wait for the page to load
        await expect(memberPage.locator('h1', { hasText: 'Pre-Order Waitlist Engine' })).toBeVisible({ timeout: 15000 });

        // Change theme
        await memberPage.locator('button', { hasText: 'Dark' }).click();

        // Check if theme changed (class name changes)
        const darkThemeButton = memberPage.locator('button', { hasText: 'Dark' });
        await expect(darkThemeButton).toHaveClass(/bg-blue-900/);
    });
});
