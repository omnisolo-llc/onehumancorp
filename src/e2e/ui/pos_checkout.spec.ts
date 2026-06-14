import { test, expect } from '@playwright/test';

test.describe('POS Checkout - Centralized Inventory', () => {
  test('Prevents double booking with Redis lock', async ({ page }) => {
    // We mock the /api/v1/payments/terminal/reserve to simulate Redis lock failure
    await page.route('/api/v1/payments/terminal/reserve', async route => {
      const json = { success: false, error_message: 'Insufficient inventory. Available: 0' };
      await route.fulfill({ json });
    });

    await page.goto('/pos/terminal');
    await page.setViewportSize({ width: 375, height: 667 });

    // Login
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    const discoverBtn = page.locator('text=Discover Readers');
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
    }

    // Simulate reader connect
    const connectBtn = page.getByRole('button', { name: 'Connect', exact: true }).first();
    if (await connectBtn.isVisible()) {
        await connectBtn.click();
    }

    // Attempt charge
    const chargeBtn = page.getByRole('button', { name: /Charge/ }).first();
    if (await chargeBtn.isVisible()) {
        await chargeBtn.click();
    }

    await expect(page.getByText('Reservation failed: Insufficient inventory. Available: 0')).toBeVisible({ timeout: 5000 });
  });

  test('Shows out of stock message when lock fails', async ({ page }) => {
    await page.route('/api/v1/payments/terminal/reserve', async route => {
      const json = { success: false, error_message: 'Item is currently being purchased elsewhere' };
      await route.fulfill({ json });
    });

    await page.goto('/pos/terminal');
    await page.setViewportSize({ width: 375, height: 667 });

    // Login
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    const discoverBtn = page.locator('text=Discover Readers');
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
    }

    // Simulate reader connect
    const connectBtn = page.getByRole('button', { name: 'Connect', exact: true }).first();
    if (await connectBtn.isVisible()) {
        await connectBtn.click();
    }

    // Attempt charge
    const chargeBtn = page.getByRole('button', { name: /Charge/ }).first();
    if (await chargeBtn.isVisible()) {
        await chargeBtn.click();
    }

    await expect(page.getByText('Reservation failed: Item is currently being purchased elsewhere')).toBeVisible({ timeout: 5000 });
  });
});
