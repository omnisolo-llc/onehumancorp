import { test, expect } from '@playwright/test';

test.describe('POS Checkout - Centralized Inventory', () => {
  test('Completes the POS flow end-to-end', async ({ page }) => {
    // Navigate to the POS terminal route
    await page.goto('/pos/terminal');
    await page.setViewportSize({ width: 375, height: 667 });

    // Wait for discover readers to appear and click
    const discoverBtn = page.getByRole('button', { name: 'Discover Readers' });
    await expect(discoverBtn).toBeVisible({ timeout: 15000 });
    await discoverBtn.click();

    // Click connect to mock reader
    const connectBtn = page.getByRole('button', { name: 'Connect' }).first();
    await expect(connectBtn).toBeVisible({ timeout: 5000 });
    await connectBtn.click();

    // The button will turn into a charge button
    const chargeBtn = page.getByRole('button', { name: /Charge \$/ });
    await expect(chargeBtn).toBeVisible({ timeout: 5000 });
    await chargeBtn.click();

    // Verify it cycles through the payment states
    await expect(page.locator('text=Payment successful!')).toBeVisible({ timeout: 15000 });
  });

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
    const connectBtn = page.locator('button', { hasText: 'Connect' }).first();
    if (await connectBtn.isVisible()) {
        await connectBtn.click();
    }

    const chargeBtn = page.locator('button', { hasText: /Charge/ });
    if (await chargeBtn.isVisible()) {
        await chargeBtn.click();
    }

    await expect(page.locator('text=Reservation failed: Insufficient inventory. Available: 0')).toBeVisible({ timeout: 5000 });
  });

  test('Shows out of stock message when lock fails', async ({ page }) => {
    await page.route('/api/v1/payments/terminal/reserve', async route => {
      const json = { success: false, error_message: 'Item is currently being purchased elsewhere' };
      await route.fulfill({ json });
    });

    await page.goto('/pos/terminal');
    await page.setViewportSize({ width: 375, height: 667 });

    const discoverBtn = page.locator('text=Discover Readers');
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
    }
    const connectBtn = page.locator('button', { hasText: 'Connect' }).first();
    if (await connectBtn.isVisible()) {
        await connectBtn.click();
    }

    const chargeBtn = page.locator('button', { hasText: /Charge/ });
    if (await chargeBtn.isVisible()) {
        await chargeBtn.click();
    }

    await expect(page.locator('text=Reservation failed: Item is currently being purchased elsewhere')).toBeVisible({ timeout: 5000 });
  });
});
