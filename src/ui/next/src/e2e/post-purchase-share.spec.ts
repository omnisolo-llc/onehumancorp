import { test, expect } from '@playwright/test';

test.describe('Post-Purchase Share Growth Loop', () => {
    test('Displays the Share & Save widget after successful checkout', async ({ page }) => {
        // Navigate to the checkout page with the success parameter
        await page.goto('/checkout?success=true&tenant=test-tenant');

        // Verify the success state is shown
        await expect(page.locator('h2', { hasText: 'Thank you for your order!' })).toBeVisible();

        // Verify the Share & Save widget is present
        await expect(page.locator('h2', { hasText: 'Share & Save' })).toBeVisible();
        await expect(page.locator('text=🎁 Give 10%, Get 10%')).toBeVisible();

        // Verify the Powered by OHC loop is present
        const footerLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(footerLink).toBeVisible();
    });
});
