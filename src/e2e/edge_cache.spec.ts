import { test, expect } from '@playwright/test';

test.describe('Edge Caching Engine CUJ', () => {
    test('Maya updates inventory, which invalidates edge cache', async ({ page }) => {
        // Step 1: Navigate to the login page (fallback to example domain if localhost server is unavailable)
        // Ensure no try/catch wrapper logic bypassing true UI validation
        await page.goto('/login');

        // As a simulated test where real backend logic might not be seeded in sandbox,
        // we'll attempt a login form fill and proceed to dashboard if available.
        const emailInput = page.locator('input[type="email"]');
        await emailInput.fill('maya@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button[type="submit"]').click();

        // Wait for dashboard
        await page.waitForURL('**/dashboard');

        // Navigate to inventory
        await page.locator('text=Inventory').click();
        await page.waitForURL('**/inventory');

        // Toggle an item to sold out
        const toggle = page.locator('.inventory-item-toggle').first();
        await toggle.click();

        // Validate success toast or state change
        await expect(page.locator('.toast-success')).toBeVisible();

        // Navigate to storefront and verify it's sold out (cache invalidated)
        await page.goto('/store/maya-bakes');
        await expect(page.locator('text=Sold Out')).toBeVisible();
    });
});
