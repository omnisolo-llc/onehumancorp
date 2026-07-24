import { test, expect } from '@playwright/test';

test.describe('Offline-Tolerant Tap-to-Pay mPOS', () => {
  test('should render Quick Charge sheet and allow cart building offline', async ({ page }) => {
    // 1. Mock the API to simulate online initial load and cache population
    await page.route('/api/v1/catalog/product', async route => {
      await route.fulfill({
        json: [
          { id: 'prod_1', name: 'Premium Coffee', price: 4.50 },
          { id: 'prod_2', name: 'Pastry', price: 3.00 }
        ]
      });
    });

    await page.goto('/pos/mpos');

    // 2. Add item to cart
    await expect(page.locator('text=Premium Coffee')).toBeVisible();
    await page.locator('text=Premium Coffee').click();

    // Check cart total updates
    await expect(page.locator('text=$4.50')).toBeVisible();

    // 3. Go Offline
    await page.context().setOffline(true);
    // Reload shouldn't be strictly necessary if PWA caching is set,
    // but we simulate offline state UI reflection.
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // 4. Add another item while offline
    await page.locator('text=Pastry').click();
    await expect(page.locator('text=$7.50')).toBeVisible();

    // 5. Open Tap to Pay (Quick Charge)
    await page.click('button[data-testid="mpos-quick-charge"]');
    await expect(page.locator('h2:has-text("Tap to Pay")')).toBeVisible();

    // We can't easily mock the full Stripe hardware SDK in standard browser e2e without deep stubs,
    // but we can verify the UI sheet opens and the total is passed correctly.
    await expect(page.locator('.text-4xl.font-bold:has-text("$7.50")')).toBeVisible();
  });
});
