import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder Viral Growth Loop', () => {
    test('Maya the Baker generates a storefront with zero clicks', async ({ page }) => {
        // We use the actual UI layout present in setup.html
        await page.goto('/setup.html');

        // Wait for the instant-bio section which contains the Zero-Click generation prompt
        const container = page.locator('#instant-bio');
        await expect(container).toBeVisible();

        // 1. Enter the business prompt
        const promptInput = page.locator('#bio-input');
        await promptInput.fill('I am a baker in Austin selling custom vegan cakes');

        // 2. Tap Generate
        const generateBtn = page.locator('#generate-storefront-btn');
        await generateBtn.click();

        // 3. Ensure the loading state is shown
        const loadingState = page.locator('#generation-loading');
        await expect(loadingState).toBeVisible();
        await expect(loadingState).toContainText('Designing catalog...');

        // 4. Ensure the live preview and next action are shown
        const previewState = page.locator('#generation-preview');
        await expect(previewState).toBeVisible({ timeout: 15000 });

        const nextAction = page.locator('#connect-bank-action');
        await expect(nextAction).toBeVisible();
    });
});
