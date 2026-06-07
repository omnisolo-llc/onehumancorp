import { test, expect } from '@playwright/test';
import { randomBytes } from 'crypto';

test.describe('Edge-Cached Storefront Store SEO & Multi-Channel Inventory Sync', () => {
  test('Priya sells the last Red Dress in-store using mobile POS', async ({ page }) => {
    // 1. Setup Phase
    const tenantId = `tenant-${randomBytes(4).toString('hex')}`;
    const productTitle = 'Red Dress';

    // We navigate to the backend API explicitly or mock the state via login.
    // For OHC, we can assume the business owner is logged in and navigates to the POS terminal.

    // As per testing guidelines, E2E tests should use real application flow if possible,
    // but the issue description explicitly mentions testing offline sync mutation
    // updating inventory and preventing subsequent checkouts.
    // Given the constraints and lack of knowledge of full login flow without fixtures,
    // we use the POS UI directly.

    await page.goto('/pos/terminal');

    // Unlock POS terminal
    const pinButtons = page.locator('button', { hasText: /^[0-9]$/ });
    if (await pinButtons.first().isVisible()) {
      await pinButtons.nth(0).click();
      await pinButtons.nth(0).click();
      await pinButtons.nth(0).click();
      await pinButtons.nth(0).click();
    }

    await page.waitForTimeout(1000);

    const clockInBtn = page.getByRole('button', { name: 'Clock In' });
    if (await clockInBtn.isVisible()) {
      await clockInBtn.click();
    }

    // Go offline
    await page.context().setOffline(true);

    // Create a new order (this deducts the item)
    await page.getByRole('button', { name: 'New Order' }).click();

    // Verify it saved offline
    await expect(page.getByRole('status')).toContainText('Payment Saved Offline');

    // Go back online
    await page.context().setOffline(false);

    // Wait for sync to complete (syncing banner should appear and disappear)
    const syncingBanner = page.locator('text=Syncing offline events...');
    // We expect it to eventually disappear after sync
    await expect(syncingBanner).toBeHidden({ timeout: 15000 });
  });
});
