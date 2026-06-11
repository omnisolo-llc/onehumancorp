import { test, expect } from '@playwright/test';

test.describe('Checkout Loyalty UI', () => {
  test('should display neighborhood loyalty points toggle', async ({ page }) => {
    // Navigate to checkout
    await page.goto('/checkout');

    // Wait for the page to load
    await page.waitForSelector('text=Secure Checkout');

    // Check if the neighborhood points section exists
    const loyaltyToggle = page.locator('text=Neighborhood Collective Points');
    await expect(loyaltyToggle).toBeVisible();

    // Verify translucent glass styling is somewhat present or text is exact
    await expect(page.locator('text=-10% off')).toBeVisible();
    await expect(page.locator('text=You have 50 points available')).toBeVisible();

    // The toggle should change background/appearance when clicked
    const toggleButton = page.locator('text=Apply points (-10% off)');
    await toggleButton.click();

    // Check if toggled state styling
    await page.waitForTimeout(500);
  });
});