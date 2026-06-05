import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory E2E', () => {
    test('should allow interaction with multiple components', async ({ page }) => {
      test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=Inventory Intelligence')).toBeVisible();
    });
});
