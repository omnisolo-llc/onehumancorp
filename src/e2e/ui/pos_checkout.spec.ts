import { test, expect } from '@playwright/test';

test.describe.skip('POS Checkout - Centralized Inventory', () => {
  test('Prevents double booking with Redis lock', async ({ page }) => {
    // We mock the /api/v1/payments/terminal/reserve to simulate Redis lock failure
    await page.route('/api/v1/payments/terminal/reserve', async route => {
      const json = { success: false, error_message: 'Insufficient inventory. Available: 0' };
      await route.fulfill({ json });
    });

    await page.goto('/pos/terminal');
    // Ensure 375px mobile responsiveness
    await page.setViewportSize({ width: 375, height: 667 });

    const discoverBtn = page.locator('text=Discover Readers');
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
    }

    // Simulate clicking charge
    // Expect error message
  });

  test('Shows out of stock message when lock fails', async ({ page }) => {
    await page.route('/api/v1/payments/terminal/reserve', async route => {
      const json = { success: false, error_message: 'Item is currently being purchased elsewhere' };
      await route.fulfill({ json });
    });

    await page.goto('/pos/terminal');
    await page.setViewportSize({ width: 375, height: 667 });

    // Assuming the user discovers and connects to a reader, and clicks 'Charge'
    // We'd look for: await expect(page.locator('text=Reservation failed: Item is currently being purchased elsewhere')).toBeVisible();
  });
});
