import { test, expect } from '@playwright/test';

test.describe('Smart Pricing E2E', () => {
    test('User can navigate to pricing settings, toggle it on, and save limits', async ({ page }) => {
        // Intercept API calls to mock success
        await page.route('**/api/v1/pricing/*/config', async route => {
            const json = { status: 'success' };
            await route.fulfill({ json });
        });

        await page.goto('/pricing-settings');

        // Check page title
        await expect(page.locator('h1')).toHaveText('Smart Pricing');

        // Find toggle and ensure it starts unchecked
        const toggle = page.locator('input[type="checkbox"]').first();
        const isChecked = await toggle.isChecked();
        if (!isChecked) {
            // Because it's sr-only, we should click its parent label or force click
            await toggle.click({ force: true });
        }

        // Adjust Minimum Price Floor
        const minPriceSlider = page.locator('input[type="range"]').first();
        await minPriceSlider.fill('600');

        // Adjust Maximum Price Ceiling
        const maxPriceSlider = page.locator('input[type="range"]').nth(1);
        await maxPriceSlider.fill('1500');

        // Verify strategies
        const maximizeRevenueCheck = page.locator('input[type="checkbox"]').nth(2); // The second strategy checkbox
        if (!(await maximizeRevenueCheck.isChecked())) {
            await maximizeRevenueCheck.click({ force: true });
        }

        // Click Save
        const saveButton = page.locator('button', { hasText: 'Save Settings' });
        await saveButton.click();

        // Verify success message
        await expect(page.locator('text=Settings saved successfully!')).toBeVisible({ timeout: 10000 });
    });
});
