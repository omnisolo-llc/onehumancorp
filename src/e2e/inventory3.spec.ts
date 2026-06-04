import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Load', () => {
    test('should load the inventory page correctly', async ({ page }) => {
<<<<<<< HEAD
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=Inventory Intelligence')).toBeVisible();
    });
});
