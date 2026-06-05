import { test, expect } from '@playwright/test';

test.describe('Pricing Page Interactive ROI Calculator Growth Loop', () => {
    test('updates ROI calculation based on user input and links to checkout', async ({ page }) => {
        // Go to pricing page
        await page.goto('/pricing');

        // Look for the ROI Calculator section
        const calculatorHeader = page.locator('h2:has-text("Calculate Your Pro Plan ROI")');
        await expect(calculatorHeader).toBeVisible();

        // Check the default state: 50 orders, $40 AOV
        // Current Revenue: 50 * 40 = $2000
        // Projected: 63 * 46 = 2898
        // Growth: 2898 - 2000 = $898
        let currentRevenue = page.locator('p:has-text("$2,000")');
        await expect(currentRevenue).toBeVisible();
        let projectedGrowth = page.locator('span:has-text("+$898")');
        await expect(projectedGrowth).toBeVisible();

        // Find the sliders
        const ordersSlider = page.locator('input[type="range"]').nth(0);

        // Update the sliders to trigger an interactive growth calculation update
        // We'll dispatch a change event via JS since Playwright's fill/type on ranges can be tricky depending on the browser
        await ordersSlider.fill('100');
        // Let React catch up
        await page.waitForTimeout(200);

        // Updated state: 100 orders, $40 AOV
        // Current Revenue: 100 * 40 = $4000
        // Projected Orders: Math.round(100 * 1.25) = 125
        // Projected AOV: 40 * 1.15 = 46
        // Projected Revenue: 125 * 46 = 5750
        // Growth: 5750 - 4000 = $1750

        currentRevenue = page.locator('p:has-text("$4,000")');
        await expect(currentRevenue).toBeVisible();
        projectedGrowth = page.locator('span:has-text("+$1,750")');
        await expect(projectedGrowth).toBeVisible();

        // Find the "Upgrade to Pro Now" button inside the ROI calculator section
        const upgradeButton = page.locator('button:has-text("Upgrade to Pro Now")');
        await expect(upgradeButton).toBeVisible();

        // Click it and verify it navigates to checkout for Pro
        await upgradeButton.click();

        // Ensure we moved to the checkout page with the correct tier param
        await expect(page).toHaveURL(/.*\/checkout\?tier=Pro/);
    });
});
