import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Load', () => {
    test('should load the inventory page correctly', async ({ page }) => {
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=Inventory Intelligence')).toBeVisible();
    });
});
