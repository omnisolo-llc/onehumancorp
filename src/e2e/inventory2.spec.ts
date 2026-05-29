import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Dismiss', () => {
    test('should allow dismissing a restock proposal', async ({ page }) => {
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=⚠️ Running low: Medium Red Dress')).toBeVisible();

        await page.click('button:has-text("Dismiss")');
        await expect(page.locator("text=No active restock proposals. You're all set!")).toBeVisible();
    });
});
