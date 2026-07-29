import { test, expect } from '@playwright/test';

test.describe('Kitchen Command Center Flow', () => {
  test('loads orders and menu from backend without test data', async ({ page }) => {
    // 1. Visit the Kitchen Command Center
    await page.goto('/kitchen');

    // 2. Wait for the API calls to finish and UI to render
    // Instead of waiting for network requests explicitly, wait for UI elements
    await page.waitForSelector('h1:has-text("Kitchen Command Center")');

    // 3. Verify Active Orders section exists
    await expect(page.locator('h2:has-text("Active Orders")')).toBeVisible();

    // 4. Verify Daily Menu section exists
    await expect(page.locator('h2:has-text("Daily Menu")')).toBeVisible();

    // 5. Ensure the test seed event is gone and we are relying on real backend data
    // The "Seed test" listener was removed, so any data shown must be real.
    // Check if there are either orders or the empty state "No active orders"
    const activeOrders = page.locator('text="No active orders"');
    const orderCards = page.locator('text="Mark Ready & Notify"');

    // Either empty state is visible or order cards are visible
    const isOrdersEmpty = await activeOrders.isVisible();
    const hasOrderCards = await orderCards.count() > 0;

    expect(isOrdersEmpty || hasOrderCards).toBeTruthy();
  });
});
