import { test, expect } from '@playwright/test';

test.describe('Omnichannel Cart and Tap-to-Pay Checkout Flow', () => {
    test.beforeEach(async ({ page }) => {
        // Authenticate as a mobile owner using the UI
        await page.goto('/login');
        await page.fill('input[name="username"]', 'priya_owner');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button:has-text("Login")');
        // We use a broader match to support various initial routing setups
        await expect(page).toHaveURL(/.*dashboard.*/);
    });

    test('should create a cart, add item, and initiate tap-to-pay', async ({ page }) => {
        // Mock viewport to simulate mobile device (375px wide)
        await page.setViewportSize({ width: 375, height: 812 });

        // Navigate to POS/New Sale area
        await page.click('button[aria-label="New Sale"]');
        await expect(page).toHaveURL(/\/pos\/cart/);

        // Wait for inventory to load and add a dress variant
        await page.waitForSelector('.product-list');
        await page.click('button[aria-label="Add Dress"]');

        // Verify cart updates
        await expect(page.locator('.cart-total')).toContainText('$50.00');

        // Proceed to Tap to Pay
        await page.click('button:has-text("Tap to Pay")');

        // System should request Stripe terminal connection and transition state
        await expect(page.locator('.terminal-status')).toContainText('Waiting for card tap...');

        // In a real e2e environment, we might click a 'simulate tap' button if we mock the native bridge
        // but for now, we just assert the cart transitions to the ready state and UI reflects it
        await page.click('button:has-text("Simulate Successful Tap")');

        // Confirm checkout completion
        await expect(page.locator('.receipt-prompt')).toBeVisible();
        await expect(page.locator('.receipt-prompt')).toContainText('Sale Completed');
    });
});
