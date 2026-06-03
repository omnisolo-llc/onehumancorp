import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory E2E', () => {
    test('Non-technical user should view AI restock alerts and stock status', async ({ page }) => {
        // We do not wait for the networkidle on this isolated component test to avoid timeouts
        // if external tracking/assets take too long, bypassing implicit fixture stalls

        // Navigate using relative URL
        await page.goto('/inventory');

        // Wait for the inventory page to load completely by checking the main heading
        await expect(page.locator('h1', { hasText: 'Inventory' })).toBeVisible({ timeout: 10000 });

        // Verify AI Alert is present
        await expect(page.locator('text=✨ Heads up Priya')).toBeVisible();

        // Verify Inventory Item is present
        await expect(page.locator('text=Blue Summer Dress (Size M)')).toBeVisible();
        await expect(page.locator('text=Low Stock')).toBeVisible();
    });
});
