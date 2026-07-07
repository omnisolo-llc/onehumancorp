import { test, expect } from '@playwright/test';
import { setupE2E, getSeededUser } from '../../../e2e/fixtures';

test.describe('Offline POS Tap to Pay Sync Flow', () => {
  setupE2E();

  test.beforeEach(async ({ page }) => {
    await page.goto('/pos/terminal');

    // Unlock the terminal if needed
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }

    const clockInBtn = page.getByRole('button', { name: 'Clock In' });
    if (await clockInBtn.isVisible()) {
        await clockInBtn.click();
    }
  });

  test('Local-First Offline-Tolerant Mobile POS flow', async ({ page, context }) => {
    // 1. User operates POS interface
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();
    await page.getByRole('button', { name: 'Discover Readers' }).click();
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // 2. Network connection drops
    await context.setOffline(true);
    await expect(page.getByText('Offline Mode - Safe to transact')).toBeVisible({ timeout: 5000 });

    // 3. Add item and click "Charge"
    // Wait for product catalog to be visible and click the first product
    await expect(page.getByText('Product Catalog')).toBeVisible({ timeout: 15000 });
    const productButton = page.locator('.grid.grid-cols-1.gap-3.mb-8 button').first();
    await productButton.click();

    await expect(page.getByRole('button', { name: /Charge \$/ })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Charge \$/ }).click();

    // 4 & 5. Logs transaction (pending_sync) and shows instant success screen
    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 5000 });

    // 6. Network restored
    await context.setOffline(false);

    // Wait for the sync to complete and the success message (this usually comes through as an agent notification or UI refresh depending on exact logic in terminal/page.tsx, but here we can check the offline indicator disappears)
    await expect(page.getByText('Online', { exact: true })).toBeVisible({ timeout: 5000 });
  });
});
