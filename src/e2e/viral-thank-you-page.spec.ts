import { expect } from '@playwright/test';
import { test, adminPage } from './fixtures';

test.describe('Viral Thank You Page Generator UI', () => {
    test('renders the viral thank you generator page and generates a link', async ({ page }) => {
        page = await adminPage(page);

        // Navigate directly to the generator page via dashboard
        await page.goto('/dashboard.html');
        await page.locator('#thank-you-loop-link').click();

        // Check for the main title
        await expect(page.locator('h1.font-outfit')).toHaveText(/Viral Thank You Page Generator/);

        // Check for the "Page Settings" and "Live Preview" panels
        await expect(page.locator('h2', { hasText: 'Page Settings' })).toBeVisible();
        await expect(page.locator('h2', { hasText: 'Live Preview' })).toBeVisible();

        // Select a theme
        await page.selectOption('#promo-theme', 'shipped');

        // Set give/get percentage
        await page.fill('#promo-give-amount', '15');
        await page.fill('#promo-get-amount', '20');

        // Verify the preview updates based on the input
        await expect(page.locator('#preview-title')).toHaveText('Order Shipped!');
        await expect(page.locator('#preview-badge')).toHaveText('Give $15, Get $20');

        // Click the generate button. It will now hit the REAL backend API instead of a mock!
        await page.click('button[data-testid="generate-promo-btn"]');

        // Wait for the result container to appear
        await expect(page.locator('#result-container')).toBeVisible();

        // Verify the generated share link is shown and valid
        const shareLinkInput = page.locator('#share-link');
        await expect(shareLinkInput).toBeVisible();

        const shareLinkValue = await shareLinkInput.inputValue();
        expect(shareLinkValue).toContain('ohc.app/thank-you');
        expect(shareLinkValue).toContain('status=shipped');
        expect(shareLinkValue).toContain('give=15');
        expect(shareLinkValue).toContain('get=20');

        // Verify the "Copy Link" button works
        const copyBtn = page.locator('#copy-btn');
        await expect(copyBtn).toHaveText('Copy Link');
        await copyBtn.click();
        await expect(copyBtn).toHaveText('Copied!');
    });
});
