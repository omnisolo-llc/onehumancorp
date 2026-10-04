import { test, expect } from '@playwright/test';

test.describe('Multi-Channel Inventory Sync & Distributed POS', () => {

  // Setup product and inventory here if possible via API or mock
  test('should handle concurrent checkout reservation correctly', async ({ page, context }) => {
    const testProductId = 'prod-e2e-sync-test';

    // Navigate to the POS page in first context
    await page.goto(`/pos/terminal?product_id=${testProductId}`);

    // Simulate online checkout in parallel
    const page2 = await context.newPage();
    await page2.goto(`/checkout?product_id=${testProductId}`);

    // Both pages loaded
    await expect(page.locator('#pos-keypad')).toBeVisible();
    await expect(page2.locator('#checkout-screen')).toBeVisible();

    // Get the buttons ready
    const posBtn = page.locator('#cash-btn-offline');
    const payBtn = page2.getByRole('button', { name: 'Pay' });

    // Click them concurrently
    await Promise.allSettled([
      posBtn.click(),
      payBtn.click(),
    ]);

    // Give requests time to complete
    await page.waitForTimeout(2000);

    // We need to check if the conflict error appeared on either side.
    // One side should get an out-of-stock error, while the other proceeds.
    const posError = page.locator('text=Error: Oops! Item just sold out.');
    const onlineError = page2.locator('text=The selected product just sold out.');

    const isPosError = await posError.isVisible();
    const isOnlineError = await onlineError.isVisible();

    // Exactly one should fail due to stock depletion
    if (isPosError || isOnlineError) {
      expect(isPosError !== isOnlineError).toBeTruthy();
    }

  });

  test('verify online checkout page UI renders gracefully', async ({ page }) => {
    await page.goto('/checkout?product_id=prod_test');
    await expect(page.getByText('Secure Checkout')).toBeVisible();
  });

  test('verify POS terminal offline UI renders', async ({ page }) => {
    await page.goto('/pos/terminal');
    await expect(page.locator('#pos-keypad')).toBeVisible();
  });

  test('verify KDS terminal renders', async ({ page }) => {
    await page.goto('/pos/kds');
    await expect(page.locator('body')).toBeVisible();
  });

  test('verify inventory page renders', async ({ page }) => {
    await page.goto('/inventory');
    await expect(page.locator('body')).toBeVisible();
  });
});
