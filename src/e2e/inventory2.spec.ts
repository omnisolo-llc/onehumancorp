import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Dismiss', () => {
    test('should allow dismissing a restock proposal', async ({ page }) => {
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=⚠️ Running low: Medium Red Dress').first()).toBeVisible({ timeout: 15000 });

        await page.locator('button:has-text("Dismiss")').first().click();
        await expect(page.locator("text=No active restock proposals. You're all set!").first()).toBeVisible({ timeout: 15000 });
    });
});
