import { expect } from '@playwright/test';
import { test, adminPage } from './fixtures';

test.describe('Viral Seasonal Promo Generator UI', () => {
    test('renders the viral seasonal promo generator page and generates a link', async ({ page }) => {
        page = await adminPage(page);

        // Navigate directly to the generator page
        await page.goto('/viral-seasonal-promo-generator.html');

        // Check for the main title
        await expect(page.locator('h1.font-outfit')).toHaveText(/Viral Seasonal Promo Generator/);

        // Check for the "Promo Settings" and "Live Preview" panels
        await expect(page.locator('h2', { hasText: 'Promo Settings' })).toBeVisible();
        await expect(page.locator('h2', { hasText: 'Live Preview' })).toBeVisible();

        // Select a theme
        await page.selectOption('#promo-theme', 'christmas');

        // Set discount percentage
        await page.fill('#promo-discount', '25');

        // Verify the preview updates based on the input
        await expect(page.locator('#preview-title')).toHaveText('Holiday Gift Event');
        await expect(page.locator('#preview-badge')).toHaveText('25% OFF');

        // Intercept API request to prevent failure
        await page.route('/api/v1/growth/seasonal-promo/generate', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ share_link: 'https://ohc.app/promo/e2e-tenant?theme=christmas&discount=25' })
            });
        });

        // Click the generate button
        await page.click('button[data-testid="generate-promo-btn"]');

        // Wait for the result container to appear
        await expect(page.locator('#result-container')).toBeVisible();

        // Verify the generated share link is shown and valid
        const shareLinkInput = page.locator('#share-link');
        await expect(shareLinkInput).toBeVisible();

        const shareLinkValue = await shareLinkInput.inputValue();
        expect(shareLinkValue).toContain('ohc.app/promo');
        expect(shareLinkValue).toContain('theme=christmas');
        expect(shareLinkValue).toContain('discount=25');

        // Verify the "Copy Link" button works (the UI interaction, since clipboard APIs are tricky in headless tests without permissions)
        const copyBtn = page.locator('#copy-btn');
        await expect(copyBtn).toHaveText('Copy Link');
        await copyBtn.click();
        await expect(copyBtn).toHaveText('Copied!');
    });
});
