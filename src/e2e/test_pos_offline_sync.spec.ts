import { test, expect } from '@playwright/test';

test.describe('Offline-Tolerant POS Terminal Checkout', () => {
  test('POS terminal queues transaction when offline and syncs when online', async ({ page, context }) => {
    // Navigate to local API directly to set up origin to allow localstorage modification
    await page.goto('/api/staff');
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Carlos',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the UI to load and auto-fetch the staff data
    await page.waitForTimeout(2000);

    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    // Click inside the body to ensure interaction context
    await page.mouse.click(10, 10);
    await page.waitForTimeout(1000);

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Verify successful login
    await expect(page.locator('text=Not Clocked In').or(page.locator('text=Clocked In'))).toBeVisible();

    // Set network to offline
    await context.setOffline(true);

    // Mock the UI to reflect offline if the native event isn't fully caught by playwright
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Ensure the Offline Mode badge is visible
    await expect(page.locator('text=Offline Mode').first()).toBeVisible();

    // Click "Quick Charge $50" while offline
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();

    // Verify it queues the order
    await expect(page.locator('text=Payment Saved Offline')).toBeVisible();

    // Assert the transaction was written to localStorage
    const queuedTxs = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(queuedTxs.length).toBeGreaterThan(0);
    expect(queuedTxs[0].amount_cents).toBe(5000);

    // Make network online

    await page.route("**/api/v1/payments/terminal/sync_offline", route => route.fulfill({ status: 200, json: { failed_transaction_ids: [] } }));
    await context.setOffline(false);

    // Fire online event to trigger page.tsx sync
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Verify "Syncing..." or Online indicator
    await expect(page.locator('text=Online').first()).toBeVisible();

    // Wait for the sync to complete and the local storage to be cleared
    await expect(async () => {
      const remainingPosTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
      expect(remainingPosTx.length).toBe(0);
    }).toPass({ timeout: 15000 });

    // Ensure the queue was cleared successfully
    const afterSyncTxs = await page.evaluate(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(afterSyncTxs.length).toBe(0);
  });
});
