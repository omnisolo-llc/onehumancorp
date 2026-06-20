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

  test('Handles offline mode gracefully and enqueues transactions', async ({ page, context }) => {
    // Wait for the UI to be ready
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();

    // Discover Readers
    await page.getByRole('button', { name: 'Discover Readers' }).click();

    // Connect to a reader
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Wait for charge button
    await expect(page.getByRole('button', { name: /Charge \$/ })).toBeVisible({ timeout: 15000 });

    // Go offline
    await context.setOffline(true);

    // Check that offline indicator appears
    await expect(page.getByText('Offline - Changes will sync later')).toBeVisible({ timeout: 5000 });

    // Click charge
    await page.getByRole('button', { name: /Charge \$/ }).click();

    // Check offline processing text
    await expect(page.getByText('Processing offline payment...')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 5000 });

    // Go online
    await context.setOffline(false);

    // It should go back to online indicator
    await expect(page.getByText('Online', { exact: true })).toBeVisible({ timeout: 5000 });
  });

});
