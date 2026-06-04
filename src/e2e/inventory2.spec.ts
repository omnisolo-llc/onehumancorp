import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Dismiss', () => {
    test('should allow dismissing a restock proposal', async ({ page }) => {
<<<<<<< HEAD
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=⚠️ Running low: Medium Red Dress')).toBeVisible();

        await page.click('button:has-text("Dismiss")');
        await expect(page.locator("text=No active restock proposals. You're all set!")).toBeVisible();
    });
});
