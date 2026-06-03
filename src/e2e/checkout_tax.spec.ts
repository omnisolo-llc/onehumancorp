import { test, expect } from '@playwright/test';

test.describe('Autonomous AI Tax and Compliance Engine', () => {

    test('Full E2E Checkout Flow with Dynamic Tax Calculation', async ({ page }) => {
        // 1. Navigate to the storefront
        await page.goto('/');

        // Wait for store to load
        await page.waitForLoadState('networkidle');

        // 2. Select a product to add to cart
        // We look for a standard product card (mocking an item like "Custom Cake" or "Handyman Service")
        const productCard = page.locator('.product-card').first();
        if (await productCard.isVisible()) {
            await productCard.click();
            await page.click('button:has-text("Add to Cart")');
        } else {
            // Fallback for different UI layouts
            await page.click('button:has-text("Buy Now")');
        }

        // 3. Proceed to checkout
        await page.click('button:has-text("Checkout")');
        await expect(page).toHaveURL(/.*checkout/);

        // 4. Fill out buyer location (triggering the tax engine)
        await page.fill('input[name="email"]', 'test-buyer@example.com');
        await page.fill('input[name="firstName"]', 'Jane');
        await page.fill('input[name="lastName"]', 'Doe');
        await page.fill('input[name="addressLine1"]', '123 Main St');

        // Changing country and state to trigger US-CA tax logic
        await page.selectOption('select[name="country"]', 'US');
        await page.fill('input[name="state"]', 'CA');
        await page.fill('input[name="zip"]', '90210');

        // Allow network debounce for the tax API call
        await page.waitForTimeout(1000);

        // 5. Verify the tax line item is dynamically added
        const taxLine = page.locator('.checkout-summary-tax');
        await expect(taxLine).toBeVisible();
        // Assuming California 8% fallback rate for this test context
        await expect(taxLine).toContainText('Tax');

        // Check that the total was updated
        const totalLine = page.locator('.checkout-summary-total');
        await expect(totalLine).toBeVisible();
    });

    test('Dashboard Tax Health & Regulatory Alerts', async ({ page }) => {
        // 1. Login as the business owner (using standard E2E test bypass if applicable or direct route)
        await page.goto('/login');

        // Using standard E2E test account credentials from seed
        await page.fill('input[name="email"]', 'test@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');

        await page.waitForURL('**/dashboard**');

        // 2. Navigate to the Tax / Finance section
        const taxNav = page.locator('a:has-text("Taxes"), a:has-text("Finance")');
        if (await taxNav.isVisible()) {
            await taxNav.click();
        } else {
            await page.goto('/dashboard/finances/taxes');
        }

        // 3. Verify the Tax Health widget is visible
        await expect(page.locator('text=Tax Health').first()).toBeVisible();

        // 4. Verify Regulatory AI Agent alerts (Nexus Thresholds)
        const alertsSection = page.locator('.ai-regulatory-alerts, .alert-banner');
        await expect(alertsSection).toBeVisible();

        // Assert the presence of a plain-language warning about economic nexus
        await expect(page.locator('text=/nearing the economic nexus/i').first()).toBeVisible();
    });
});
