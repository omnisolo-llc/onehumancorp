import { test, expect } from './fixtures';

test.describe('Offline-Tolerant POS Terminal Checkout', () => {
  test('POS terminal queues transaction when offline and syncs when online', async ({ page, context }) => {
    // Navigate to the POS Terminal page
    await page.goto('/pos/terminal');

    // Wait for the body to be visible
    await expect(page.locator('body')).toBeVisible();

    // Check if the pin pad is visible, if so tap digits
    const pinPadVisible = await page.getByRole('button', { name: '1', exact: true }).isVisible().catch(() => false);
    if (pinPadVisible) {
      await page.getByRole('button', { name: '1' }).click();
      await page.getByRole('button', { name: '2' }).click();
      await page.getByRole('button', { name: '3' }).click();
      await page.getByRole('button', { name: '4' }).click();
    }

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

    // Click "New Order" while offline
    await page.getByRole('button', { name: 'New Order' }).click();

    // Verify it queues the order
    await expect(page.locator('text=Payment Saved Offline')).toBeVisible();

    // Assert the transaction was written to localStorage
    const queuedTxs = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(queuedTxs.length).toBeGreaterThan(0);

    // Make network online
    await context.setOffline(false);

    // Fire online event to trigger page.tsx sync
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Wait for the sync to complete and the local storage to be cleared
    await page.waitForFunction(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]').length === 0;
    }, { timeout: 15000 });

    // Ensure the queue was cleared successfully
    const afterSyncTxs = await page.evaluate(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(afterSyncTxs.length).toBe(0);
  });
});
