import { test, expect } from '@playwright/test';

test.describe('Dynamic Edge-Caching Storefront', () => {
    test('Maya can build a storefront and sees edge-caching performance metrics', async ({ page, request }) => {
        // We will intercept the initial storefront builder page load because the builder fetches current state
        // Let's go to storefront-builder
        // IMPORTANT: Playwright tests running under bazel test usually get the base URL automatically injected.
        // If it's missing, default to hitting the root or relative paths.
        await page.goto('/storefront-builder');

        // Wait for bio input to be visible and type a description
        const textarea = page.locator('#bio-input');
        await expect(textarea).toBeVisible({ timeout: 15000 });
        await textarea.fill('I bake custom vegan cakes in Portland.');

        // Wait for the Build My Storefront button and click it
        const generateBtn = page.locator('#generate-btn');
        await expect(generateBtn).toBeEnabled({ timeout: 5000 });
        await generateBtn.click();

        // The UI should switch to generating, and then eventually show the preview blocks
        // The mock backend endpoint `/api/v1/builder/generate` handles the response
        const launchBtn = page.locator('#launch-btn');

        // The page simulates launching the store when launchBtn is clicked.
        // `waitFor` the button to become visible (status goes to draft or idle preview).
        await expect(launchBtn).toBeVisible({ timeout: 30000 });
        await launchBtn.click();

        // Verify the success "Live" screen appears.
        await expect(page.locator('h1:has-text("You\'re Live!")')).toBeVisible({ timeout: 15000 });

        // Verify the "Store Performance" card is visible.
        const performanceCardTitle = page.locator('span', { hasText: 'Store Performance' });
        await expect(performanceCardTitle).toBeVisible();

        const performanceValue = page.locator('div', { hasText: '< 50ms' });
        await expect(performanceValue).toBeVisible();

        // Test the Operations Agent backend cache invalidation logic
        // by pushing an event to the backend endpoint.
        const productUpdateResponse = await request.post('/api/v1/catalog/product', {
            data: {
                name: 'Vegan Chocolate Cake',
                price: '$25.00',
                description: 'Delicious edge-cached cake.',
                item_type: 'physical'
            }
        });

        expect(productUpdateResponse.ok()).toBeTruthy();
    });
});
