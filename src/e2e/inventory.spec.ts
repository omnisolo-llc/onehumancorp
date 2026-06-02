import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory', () => {
    test('should show predictive restock proposals and allow 1-tap approval', async ({ page }) => {
        // Must start from home page per guidelines
        await page.goto('/');

        // Navigate to inventory
        await page.goto('/inventory');

        // Wait for proposal to load
        await expect(page.locator('text=⚠️ Running low: Medium Red Dress')).toBeVisible();
        await expect(page.locator('text=You will sell out in 3 days. Restock 20 units for $150?')).toBeVisible();

        // 1-Tap Approve
        await page.click('button:has-text("Approve Restock ($150)")');

        // Verify successful approval removes the proposal
        await expect(page.locator("text=No active restock proposals. You're all set!")).toBeVisible();
    });
});
