import { test, expect } from '@playwright/test';
import { e2eEnvironment } from './tests/e2e-env';

test.describe('Agentic Multi-Currency & Localized Checkout', () => {
  test('CUJ: Boutique owner displays products in GBP and checks out in GBP', async ({ page }) => {
    await e2eEnvironment.setupTenantAndLogin(page, 'tenant-priya');

    // 1. Owner logs in and sets up a new product (USD base)
    await page.goto('/dashboard/products');
    await page.click('text=Add Product');
    await page.fill('input[name="title"]', 'Cashmere Sweater');
    await page.fill('input[name="price"]', '100'); // 100 USD
    await page.click('button[type="submit"]');
    await expect(page.locator('text=Cashmere Sweater')).toBeVisible();

    // 2. Simulated UK buyer visits the storefront (with target_currency=GBP)
    await page.goto('/store/tenant-priya?target_currency=GBP&target_region=UK');

    // The FX service applies a rate (e.g., 0.75 for USD->GBP). So 100 USD -> 75 GBP.
    // Also, UI should reflect £ or GBP.
    await expect(page.locator('text=Cashmere Sweater')).toBeVisible();

    // We expect the price to be adjusted.
    // If rate is exactly 0.75, then 100 * 0.75 = 75.
    // Depending on the UI formatting, we might just look for the number or currency.
    // This assumes the frontend fetches from /api/catalog/products?target_currency=GBP
    await expect(page.locator('.product-price')).toContainText('75');

    // 3. Buyer adds to cart and proceeds to checkout
    await page.click('text=Add to Cart');
    await page.click('text=Checkout');

    // 4. Verification that checkout session contains GBP and applied UK tax (20%)
    await expect(page.locator('.checkout-total')).toBeVisible();
    await expect(page.locator('.checkout-total')).toContainText('90'); // 75 GBP + 20% VAT = 90 GBP

    // 5. Complete checkout
    await page.click('text=Confirm Order');
    await expect(page.locator('text=Order Confirmed')).toBeVisible();

    // 6. Return to owner dashboard, check invoice/order
    await page.goto('/dashboard/orders');
    await expect(page.locator('text=Cashmere Sweater')).toBeVisible();
    await expect(page.locator('text=GBP')).toBeVisible();
  });
});
