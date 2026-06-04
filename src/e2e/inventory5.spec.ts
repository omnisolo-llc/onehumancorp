import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory E2E', () => {
    test('should allow interaction with multiple components', async ({ page }) => {
<<<<<<< HEAD
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=Inventory Intelligence')).toBeVisible();
    });
});
