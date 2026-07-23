import { test, expect } from '@playwright/test';

test.describe('Offline-Tolerant Tap-to-Pay mPOS', () => {
  test('should render Quick Charge sheet and allow cart building offline', async ({ page }) => {
    await page.goto('/pos/mpos');

    // 1. Add item to cart
    await expect(page.locator('text=Premium Coffee')).toBeVisible();
    await page.locator('text=Premium Coffee').click();

    // Check cart total updates
    await expect(page.locator('text=$4.50')).toBeVisible();

    // 2. Go Offline
    await page.context().setOffline(true);

    // 3. Add another item while offline
    await page.locator('text=Pastry').click();
    await expect(page.locator('text=$7.50')).toBeVisible();

    // 4. Open Tap to Pay (Quick Charge)
    await page.click('button[data-testid="mpos-quick-charge"]');
    await expect(page.locator('h2:has-text("Tap to Pay")')).toBeVisible();

    // Verify the UI sheet opens and the total is passed correctly.
    await expect(page.locator('.text-4xl.font-bold:has-text("$7.50")')).toBeVisible();
  });
});
