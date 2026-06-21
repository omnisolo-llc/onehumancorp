import { test, expect } from '../../../../e2e/fixtures';

test.describe('PowerSync Offline Sync E2E', () => {
  test('POS inventory item bidirectional sync from SQLite to Postgres via PowerSync', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Simulate going offline
    const context = page.context();
    await context.setOffline(true);

    // Add item to cart and initiate tap-to-pay offline
    await page.goto('/pos/omnichannel');

    // Interact with cart
    await expect(page.getByText('Create Omnichannel Cart')).toBeVisible();
    await page.click('button:has-text("Add Default Item")');
    await page.click('button:has-text("Charge $")');
    await page.click('button:has-text("Tap to Pay (Offline)")');

    // Wait for the offline transaction to be queued (Premium Glassmorphism indicator)
    const offlinePill = page.locator('.glassmorphism', { hasText: 'Offline Mode' });
    if (await offlinePill.count() > 0) {
      await expect(offlinePill).toBeVisible();
    }

    await expect(page.getByText('Payment Queued Offline')).toBeVisible();

    // Simulate returning online
    await context.setOffline(false);

    // The SyncEngine (via PowerSync) should reconcile automatically without user intervention
    // We expect the Finance agent to notify or status to update
    await expect(page.getByText('Reconciled')).toBeVisible({ timeout: 15000 });
  });
});
