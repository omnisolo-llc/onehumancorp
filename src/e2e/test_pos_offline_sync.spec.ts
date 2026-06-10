import { test, expect } from './fixtures';

test.describe('Offline-Tolerant POS Terminal Checkout', () => {
  test('POS terminal queues transaction when offline and syncs when online', async ({ memberPage, context }) => {
    // Navigate to the POS Terminal page
    await memberPage.goto('/pos/terminal');

    // Enter PIN (1234 is commonly used, we just tap 4 digits)
    await memberPage.getByRole('button', { name: '1' }).click();
    await memberPage.getByRole('button', { name: '2' }).click();
    await memberPage.getByRole('button', { name: '3' }).click();
    await memberPage.getByRole('button', { name: '4' }).click();

    // Verify successful login
    await expect(memberPage.locator('text=Not Clocked In').or(memberPage.locator('text=Clocked In'))).toBeVisible();

    // Set network to offline
    await context.setOffline(true);

    // Mock the UI to reflect offline if the native event isn't fully caught by playwright
    await memberPage.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Ensure the Offline Mode badge is visible
    await expect(memberPage.locator('text=Offline Mode').first()).toBeVisible();

    // Click "New Order" while offline
    await memberPage.getByRole('button', { name: 'New Order' }).click();

    // Verify it queues the order
    await expect(memberPage.locator('text=Payment Saved Offline')).toBeVisible();

    // Assert the transaction was written to localStorage
    const queuedTxs = await memberPage.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(queuedTxs.length).toBeGreaterThan(0);
    expect(queuedTxs[0].amount_cents).toBe(5000);

    // Make network online
    await context.setOffline(false);

    // Fire online event to trigger page.tsx sync
    await memberPage.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Verify "Syncing..." or Online indicator
    await expect(memberPage.locator('text=Online').first()).toBeVisible();

    // Wait for the sync to complete and the local storage to be cleared
    await memberPage.waitForFunction(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]').length === 0;
    }, { timeout: 15000 });

    // Ensure the queue was cleared successfully
    const afterSyncTxs = await memberPage.evaluate(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(afterSyncTxs.length).toBe(0);
  });
});
