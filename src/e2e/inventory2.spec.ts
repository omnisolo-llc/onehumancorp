import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Dismiss', () => {
    test('should allow dismissing a restock proposal', async ({ page }) => {
      test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=⚠️ Running low: Medium Red Dress')).toBeVisible();

        await page.click('button:has-text("Dismiss")');
        await expect(page.locator("text=No active restock proposals. You're all set!")).toBeVisible();
    });
});
