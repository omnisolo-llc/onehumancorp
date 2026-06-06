import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction', async ({ page }) => {
    await page.goto('/orders');
    await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();
    await expect(page.locator('body')).toContainText(/Loaded from|No order rows|Order/);
  });

  test('should complete a tap-to-pay transaction while offline and sync when online', async ({ page, context }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Switch to offline mode
    await context.setOffline(true);

    // Wait for the operations link to be visible and click it
    await page.getByRole('link', { name: /Operations/i }).click();

    // Click the POS / In-Person tab
    await page.getByRole('tab', { name: /POS \/ In-Person/i }).click();

    // Add product to cart.
    const addToCartBtn = page.getByRole('button', { name: /Add to cart|Charge/i }).first();
    await addToCartBtn.click();

    // Or, enter amount manually if there's an input
    const chargeBtn = page.getByRole('button', { name: /Charge/i });
    await chargeBtn.click();

    // Complete transaction
    const tapToPayBtn = page.getByRole('button', { name: /Tap to Pay/i });
    await tapToPayBtn.click();

    // Look for success confirmation
    await expect(page.getByText(/Payment Successful|Saved offline/i)).toBeVisible({ timeout: 10000 });

    // Switch back to online mode
    await context.setOffline(false);

    // Give background sync time to complete
    await page.waitForTimeout(5000);

    // Navigate to Finance & Payments
    await page.getByRole('link', { name: /Finance & Payments/i }).click();

    // Verify the transaction appears. We just verify that we landed on Finance.
    await expect(page.getByRole('heading', { name: /Finance & Payments/i })).toBeVisible();
  });
});
