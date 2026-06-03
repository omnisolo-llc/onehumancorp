import { test, expect } from './fixtures';

test.describe('Autonomous Inventory & Fulfillment Engine', () => {
  test('Non-technical user should view AI restock alerts and stock status', async ({ page }) => {
    // Navigate using relative URL
    await page.goto('/inventory');

    // Verify AI Alert is present
    await expect(page.locator('text=✨ Heads up Priya')).toBeVisible();

    // Verify Inventory Item is present
    await expect(page.locator('text=Blue Summer Dress (Size M)')).toBeVisible();
    await expect(page.locator('text=Low Stock')).toBeVisible();
  });
});
