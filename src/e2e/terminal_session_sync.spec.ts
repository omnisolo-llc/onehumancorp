import { test, expect } from '@playwright/test';
import { randomBytes } from 'crypto';

test.describe('Terminal Session Offline Sync & Reconciliation', () => {
  test('Priya starts a terminal session, goes offline, syncs back, and reconciles', async ({ page }) => {
    // Navigate to POS terminal
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

    // Start Session
    const startSessionBtn = page.getByRole('button', { name: 'Start Session' });
    if (await startSessionBtn.isVisible()) {
      await startSessionBtn.click();
    }

    // Verify Terminal Status is Online
    await expect(page.locator('text=Terminal Status: Online')).toBeVisible();

    // Go offline
    await page.context().setOffline(true);
    await expect(page.locator('text=Terminal Status: Offline')).toBeVisible();

    // Create a new order while offline
    await page.getByRole('button', { name: 'New Order' }).click();
    await page.getByText('Item A').click();
    await page.getByRole('button', { name: 'Charge' }).click();

    // Verify it saved offline
    await expect(page.getByRole('status')).toContainText('Payment Saved Offline');
    await expect(page.locator('text=Pending Sync: 1')).toBeVisible();

    // Go back online
    await page.context().setOffline(false);
    await expect(page.locator('text=Terminal Status: Syncing')).toBeVisible();

    // Wait for sync to complete
    await expect(page.locator('text=Terminal Status: Online')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Pending Sync: 0')).toBeVisible();

    // End session (Reconcile)
    await page.getByRole('button', { name: 'End Session' }).click();
    await expect(page.locator('text=Session Status: RECONCILED')).toBeVisible();

    // Verify in dashboard/orders that the transaction exists
    await page.goto('/orders');
    await expect(page.locator('text=Item A')).toBeVisible();
  });
});
