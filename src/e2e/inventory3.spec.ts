import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Load', () => {
    test('should load the inventory page correctly', async ({ page }) => {
        await page.goto('/');
        await page.goto('/inventory');

        await expect(page.locator('text=Inventory Intelligence')).toBeVisible();
    });
});
