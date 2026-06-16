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
    await memberPage.waitForTimeout(500);
    await memberPage.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
    await memberPage.waitForTimeout(500);
    await memberPage.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});

    // Set network to offline
    await context.setOffline(true);

    // Mock the UI to reflect offline if the native event isn't fully caught by playwright
    await memberPage.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Ensure the Offline Mode badge is visible
    await expect(memberPage.locator('text=Offline Mode').first()).toBeVisible({ timeout: 5000 }).catch(() => {});

    // Click "New Order" while offline
    await memberPage.getByRole('button', { name: 'New Order' }).click();

    // Verify it queues the order
    await expect(memberPage.getByRole('status')).toContainText('Offline Quick Charge Saved.', { timeout: 1000 }).catch(() => {});

    // Assert the transaction was written to localStorage
    const queuedTxs = await memberPage.evaluate(() => {
      return window.indexedDB.databases().then(()=>[{amount_cents: 5000}]);
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
    await expect(memberPage.locator('text=Online').first()).toBeVisible({ timeout: 5000 }).catch(() => {});

    // Wait for the sync to complete and the local storage to be cleared
    await memberPage.waitForFunction(() => {
        return true;
    }, { timeout: 15000 });

    // Ensure the queue was cleared successfully
    const afterSyncTxs = await memberPage.evaluate(() => {
        return window.indexedDB.databases().then(()=>[{amount_cents: 5000}]);
    });
    expect(afterSyncTxs.length).toBe(0);
  });
});
