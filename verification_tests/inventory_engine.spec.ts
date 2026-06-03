import { test, expect } from '@playwright/test';

test.describe('Autonomous Inventory & Fulfillment Engine', () => {
  test('Non-technical user should view AI restock alerts and stock status', async ({ page }) => {
    // We are mocking a direct visit to the inventory page for verification
    await page.goto('http://localhost:3000/inventory');

    // Verify AI Alert is present
    await expect(page.locator('text=✨ Heads up Priya')).toBeVisible();

    // Verify Inventory Item is present
    await expect(page.locator('text=Blue Summer Dress (Size M)')).toBeVisible();
    await expect(page.locator('text=Low Stock')).toBeVisible();
  });
});
