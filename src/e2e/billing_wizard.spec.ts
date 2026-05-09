import { test, expect } from '@playwright/test';

test.describe('Billing & Credits Wizard E2E', () => {
    test.beforeEach(async ({ page }) => {
        // Start from the home page
        await page.goto('/');

        // Login as the user
        const loginEmailInput = page.locator('input[type="email"]');
        const loginPasswordInput = page.locator('input[type="password"]');
        const loginButton = page.locator('button:has-text("Login")');

        await loginEmailInput.fill('test@example.com');
        await loginPasswordInput.fill('password123');
        await loginButton.click();

        // Wait for dashboard to load completely
        await expect(page.locator('text="Your business, live in minutes."')).toBeVisible({ timeout: 10000 });

        // Wait out animations
        await page.waitForTimeout(1000);
    });

    test('should navigate full Billing & Credits wizard flow with progressive disclosure', async ({ page }) => {
        // 1. Trigger the "Billing & credits wizard" from dashboard
        const billingBtn = page.locator('button:has-text("Billing")');
        if (!await billingBtn.isVisible()) {
            const menuToggle = page.locator('button:has-text("Menu")');
            if (await menuToggle.isVisible()) {
                await menuToggle.click();
            }
        }
        await billingBtn.click();

        // 2. Verify we are on Step 0: "What does this cost?" Flow
        await expect(page.locator('text="Pricing & Billing"').first()).toBeVisible();
        await expect(page.locator('text="Your Current Usage"')).toBeVisible();
        await expect(page.locator('text="Projected Cost this Month"')).toBeVisible();

        // Check Simple Mode (default)
        await expect(page.locator('text="Raw Telemetry Config"')).not.toBeVisible();

        // 3. Toggle Advanced Mode
        await page.locator('.slint-touch-area').filter({ hasText: 'Expert Mode' }).click().catch(() => page.click('text="Expert Mode"'));

        // We expect raw JSON fields to appear
        await expect(page.locator('text="Raw Telemetry Config"')).toBeVisible();

        // Toggle back to Simple Mode
        await page.click('text="Expert Mode"');

        // 4. Progress to Step 1: "Upgrade Plan" (Plans)
        await page.click('text="View Upgrade Plans"');

        // Verify we are on Plans step
        await expect(page.locator('text="Upgrade Plan"')).toBeVisible();
        await expect(page.locator('text="Free"').first()).toBeVisible();
        await expect(page.locator('text="Pro"').first()).toBeVisible();

        // Verify Simple Mode in Step 1
        await expect(page.locator('text="Raw Billing Plan Limits"')).not.toBeVisible();

        // 5. Toggle Advanced Mode in Step 1
        await page.click('text="Expert Mode"');

        // Verify raw JSON fields in Step 1
        await expect(page.locator('text="Raw Billing Plan Limits"')).toBeVisible();

        // Verify it works
        await expect(page.locator('text="Start Free"').first()).toBeVisible();
    });
});
