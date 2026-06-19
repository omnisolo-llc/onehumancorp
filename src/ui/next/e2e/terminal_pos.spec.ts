import { test, expect } from '@playwright/test';
import { setupE2E, getSeededUser } from '../../../e2e/fixtures';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {
  setupE2E();

  test.beforeEach(async ({ page }) => {
    // Navigate to POS terminal path
    await page.goto('/pos/terminal');

    // Unlock the terminal
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
  });

  test('Processes tap-to-pay and reserves inventory with multi-item cart', async ({ page }) => {
    // Wait for the catalog to load
    await expect(page.getByText('Catalog')).toBeVisible();

    // Add first product to cart
    await page.getByRole('button').filter({ hasText: 'Silk Summer Dress' }).click();

    // Add second product to cart
    await page.getByRole('button').filter({ hasText: 'Leather Tote Bag' }).click();

    // Check cart count (should be 2)
    const cartButton = page.locator('button:has(svg.io-cart-outline)');
    await expect(page.getByText('2', { exact: true })).toBeVisible();

    // Open Cart Drawer
    await cartButton.click();
    await expect(page.getByText('Current Cart')).toBeVisible();

    // Check total
    await expect(page.getByText('$205.00')).toBeVisible();

    // Discover Readers in the drawer
    await page.getByRole('button', { name: 'Setup Card Reader' }).click();

    // Connect to a reader
    await page.getByRole('button', { name: 'Tap to Connect' }).first().click();

    // Wait for charge button in drawer
    await expect(page.getByRole('button', { name: /Charge \$/ })).toBeVisible({ timeout: 15000 });

    // Click charge
    await page.getByRole('button', { name: /Charge \$/ }).click();

    // Payment processing text checks
    await expect(page.getByText('Success!')).toBeVisible({ timeout: 20000 });

    // Cart should be empty after success
    await expect(page.getByText('Cart is empty')).toBeVisible();
  });

  test('Works in offline mode and enqueues transactions', async ({ page, context }) => {
    await expect(page.getByText('Catalog')).toBeVisible();

    // Go offline
    await context.setOffline(true);
    await expect(page.getByText('Offline')).toBeVisible();

    // Add item
    await page.getByRole('button').filter({ hasText: 'Silk Summer Dress' }).click();

    // Open Cart
    await page.locator('button:has(svg.io-cart-outline)').click();

    // Discover (simulated)
    await page.getByRole('button', { name: 'Setup Card Reader' }).click();
    await page.getByRole('button', { name: 'Tap to Connect' }).first().click();

    // Charge
    await page.getByRole('button', { name: /Charge \$/ }).click();

    // Check for saved offline status
    await expect(page.getByText('Saved Offline')).toBeVisible();

    // Go back online
    await context.setOffline(false);

    // Check for syncing indicator
    await expect(page.getByText('Syncing Transactions')).toBeVisible();
  });
});
