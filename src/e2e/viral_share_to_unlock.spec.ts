import { test, expect } from '@playwright/test';

test.describe('Viral Growth Loop - Share to Unlock Widget', () => {
    test('generator UI creates correct embed link and preview', async ({ page }) => {
        // We use page directly to bypass login since generator is a static/public tool
        // or we can test the generator output directly.
        await page.goto('/ui/share-to-unlock-generator.html');

        await expect(page.locator('h1')).toHaveText('Share-to-Unlock Generator');

        // Fill in details
        await page.fill('#title', 'Super Secret Holiday Promo');
        await page.fill('#reward', '50% off all cakes');
        await page.fill('#code', 'CAKE50');

        // Verify preview reflects changes
        await expect(page.locator('#preview-title')).toHaveText('Super Secret Holiday Promo');
        await expect(page.locator('#preview-reward-text')).toHaveText('50% off all cakes');

        // Generate Link
        await page.click('#generate-btn');

        // Check link result
        const generatedLink = await page.inputValue('#generated-url');
        expect(generatedLink).toContain('title=Super+Secret+Holiday+Promo');
        expect(generatedLink).toContain('code=CAKE50');
    });

    test('consumer widget renders and handles unlock', async ({ page }) => {
        const url = '/ui/share-to-unlock/index.html?title=Free%20Cookie&reward=One%20free%20cookie&code=FREECOOKIE';
        await page.goto(url);

        await expect(page.locator('#campaign-title')).toHaveText('Free Cookie');
        await expect(page.locator('#reward-desc')).toHaveText('One free cookie');

        // Initially locked
        await expect(page.locator('#locked-badge')).toBeVisible();

        // Simulate click to share which triggers unlock
        await page.click('#share-wa-btn');

        // The timeout is 300ms in JS, wait for unlock
        await page.waitForTimeout(500);

        // Code should now be visible and unlocked
        await expect(page.locator('#unlocked-actions')).toBeVisible();
        await expect(page.locator('#copy-code-btn')).toBeVisible();
        await expect(page.locator('#discount-code')).toHaveClass(/unlocked/);

        // Verify footer branding
        await expect(page.locator('footer a')).toHaveText('⚡ Powered by OHC');
    });
});
