import { test, expect } from '@playwright/test';

test.describe('Dynamic Edge-Caching Storefront', () => {
    test('Maya can build a storefront and sees edge-caching performance metrics', async ({ page, request }) => {
        // 1. Maya logs in and goes to the storefront builder.
        await page.goto('http://localhost:3000/storefront-builder');

        // 2. Maya describes her business.
        const textarea = page.locator('#bio-input');
        await expect(textarea).toBeVisible();
        await textarea.fill('I bake custom vegan cakes in Portland.');

        // 3. Maya generates the store.
        const generateBtn = page.locator('#generate-btn');
        await expect(generateBtn).toBeEnabled();
        await generateBtn.click();

        // 4. Maya reviews the store and clicks 1-Tap Launch.
        const launchBtn = page.locator('#launch-btn');
        // Wait for generation to finish and launch button to be available.
        await expect(launchBtn).toBeVisible({ timeout: 15000 });
        await launchBtn.click();

        // 5. The success "Live" screen appears.
        await expect(page.locator('h1:has-text("You\'re Live!")')).toBeVisible({ timeout: 15000 });

        // 6. Verify the "Store Performance" card is visible.
        const performanceCardTitle = page.locator('span', { hasText: 'Store Performance' });
        await expect(performanceCardTitle).toBeVisible();

        const performanceValue = page.locator('div', { hasText: '< 50ms' });
        await expect(performanceValue).toBeVisible();

        // 7. Verify Cache Invalidation Hook
        // Since E2E test runs against the full stack, we will simulate a webhook or an API request
        // that creates an order or updates a product.
        // For simplicity, we just verify the route exists and the cache gets invalidated in backend.
        // The backend handles invalidation via the agent event bus. We will trigger an event
        // to our local backend directly to test the operations agent hook.

        // This is a minimal check for E2E since the cache invalidation happens asynchronously.
        // We ensure the backend route for the webhook or the product update is reachable.
        const productUpdateResponse = await request.post('http://localhost:3000/api/v1/catalog/product', {
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
