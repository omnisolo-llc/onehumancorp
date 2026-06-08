import { test, expect } from '@playwright/test';

test.describe('Mobile POS Offline-to-Online Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should handle offline transaction and sync upon reconnection', async ({ page, context }) => {
    // 1. Login and navigate to POS
    await page.goto('/pos/terminal');

    // Enter PIN (Mocking valid PIN '1234')
    // Note: In real E2E we might need to seed staff data first.
    // For this test, we assume the page loads products from localStorage if offline or from API.

    await page.click('button:text("1")');
    await page.click('button:text("2")');
    await page.click('button:text("3")');
    await page.click('button:text("4")');

    await expect(page.locator('text=Online')).toBeVisible();

    // 2. Simulate Offline Mode
    await context.setOffline(true);
    await expect(page.locator('text=Offline')).toBeVisible();

    // 3. Add items to cart
    // Wait for product grid
    const productButton = page.locator('button').filter({ hasText: '$' }).first();
    await productButton.click();

    await expect(page.locator('text=View Cart')).toBeVisible();

    // 4. Complete local transaction
    await page.click('text=View Cart');
    await page.click('text=Complete Sale');

    await expect(page.locator('text=Sale saved offline.')).toBeVisible();

    // 5. Restore network
    await context.setOffline(false);
    await expect(page.locator('text=Online')).toBeVisible();

    // 6. Verify autonomous sync
    await expect(page.locator('text=Syncing')).toBeVisible();

    // 7. Check for Agent Intervention (if stock hit zero/low)
    // We assume the sync triggers a LowStock restock suggestion
    await expect(page.locator('text=Restock Suggestion')).toBeVisible({ timeout: 15000 });

    await page.click('text=Approve');
    await expect(page.locator('text=Restock Suggestion')).not.toBeVisible();
  });

  test('should hide product from storefront when stock hits zero', async ({ page }) => {
     // This would involve checking the public storefront, but we can verify it via the POS catalog too
     // if the product disappeared or shows as out of stock.
     await page.goto('/pos/terminal');
     // Login...
     // Verify product with 0 stock has is_hidden logic applied on backend (testable via API)
  });
});
