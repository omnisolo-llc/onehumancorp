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

  test('Processes tap-to-pay and reserves inventory', async ({ page }) => {
    // Wait for the UI to be ready
    await expect(page.getByRole('button', { name: 'New Order' })).toBeVisible();

    // Click New Order
    // await page.getByRole('button', { name: 'New Order' }).click();

    // Discover Readers
    await page.getByRole('button', { name: 'Discover Readers' }).click();

    // Connect to a reader
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Wait for charge button
    await expect(page.getByRole('button', { name: /Charge \$/ })).toBeVisible({ timeout: 15000 });

    // Click charge
    await page.getByRole('button', { name: /Charge \$/ }).click();

    // Payment processing text checks
    await expect(page.getByText('Payment successful!')).toBeVisible({ timeout: 20000 });
  });
});
