import { test, expect } from '@playwright/test';

test.describe('Branding Growth Loop', () => {
    test('Powered by OHC footer is present and links correctly', async ({ page }) => {
        // Go to a simulated storefront builder preview which renders the blocks.
        // The storefront-builder page renders the blocks from localstorage.
<<<<<<< HEAD
        await page.goto('/storefront-builder');
=======
        await page.goto('http://localhost:3000/storefront-builder');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
        await page.evaluate(() => localStorage.setItem('ohc_builder_status', 'draft'));
        await page.reload();

        // We can test the presence of the footer directly
        const footerLink = page.locator('a[href^="ohc://join?ref="]');
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toContainText('Powered by');
        await expect(footerLink).toContainText('OHC');
    });

    test('Website Builder also shows Powered by OHC footer', async ({ page }) => {
<<<<<<< HEAD
        await page.goto('/website-builder');
=======
        await page.goto('http://localhost:3000/website-builder');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
        await page.evaluate(() => localStorage.setItem('ohc_builder_status', 'draft'));
        await page.reload();
        const footerLink = page.locator('a[href^="ohc://join?ref="]');
        await expect(footerLink).toBeVisible();
    });
});
