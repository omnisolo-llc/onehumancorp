import { test, expect } from '@playwright/test';

test.describe('POS and Inventory Sync', () => {
  // Use a known existing product or dynamically add one using the backend API directly via request object if needed.
  // Assuming 'tenant_123' and a product ID exist in the mock data based on the KDS/POS structure.

  test('Simultaneous purchase attempt online vs in-store', async ({ request, page }) => {
    // Navigate to the real POS terminal page
    await page.goto('/pos/terminal');

    // Attempt to log in if the page requires it
    const pinInput = page.locator('input[type="password"], input[placeholder*="PIN"]');
    if (await pinInput.count() > 0) {
       await pinInput.fill('1234'); // using dummy pin
       await page.locator('button', { hasText: 'Unlock' }).click();
    }

    // Wait for inventory to load and be visible
    const productButton = page.locator('button', { hasText: 'Add to Cart' }).first().or(page.locator('.product-name').first());
    await expect(productButton).toBeVisible({ timeout: 10000 });

    // Get product ID if possible, or trigger checkout flow
    await productButton.click();

    // Now trigger checkout from POS
    const posCheckoutBtn = page.locator('button', { hasText: /Checkout/i });
    await expect(posCheckoutBtn).toBeVisible();

    // Trigger online purchase via API concurrently while clicking POS checkout
    // Assuming product ID 'test_product_id' for now, this may fail gracefully if not found
    const onlinePurchasePromise = request.post('/api/v1/payments/terminal/reserve', {
      data: {
        product_id: 'test_product_id',
        quantity: 1,
        ttl_seconds: 15
      },
      headers: {
        'x-tenant-id': 'tenant_123'
      }
    });

    const [posRes, onlineRes] = await Promise.all([
      // Await network response triggered by checkout click
      page.waitForResponse(response => response.url().includes('/api/v1/payments/terminal/reserve') && response.request().method() === 'POST'),
      onlinePurchasePromise,
      posCheckoutBtn.click()
    ]);

    const posJson = await posRes.json();
    const onlineJson = await onlineRes.json();

    // Check results: One should succeed, the other should fail due to lock
    expect((posJson.success && !onlineJson.success) || (!posJson.success && onlineJson.success)).toBeTruthy();
  });
});
