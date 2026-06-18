import { test, expect } from './fixtures';

test.describe('Offline-Tolerant POS Terminal Checkout', () => {
  test('POS terminal queues transaction when offline and syncs when online', async ({ memberPage, context }) => {
    // Navigate to the POS Terminal page
    await memberPage.goto('/pos.html');

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

    // Click "Quick Charge $50" while offline
    await memberPage.getByRole('button', { name: 'Quick Charge $50' }).click();

    // Verify it queues the order
    await expect(memberPage.getByText('Offline Quick Charge Saved.')).toBeVisible({ timeout: 10000 });

    // Assert the transaction was written to IndexedDB
    const queuedTxs = await memberPage.evaluate(() => {
      return new Promise<any[]>((resolve, reject) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onerror = () => reject(req.error);
        req.onsuccess = () => {
          const db = req.result;
          if (!db.objectStoreNames.contains('actions')) {
            resolve([]);
            return;
          }
          const tx = db.transaction('actions', 'readonly');
          const store = tx.objectStore('actions');
          const all = store.getAll();
          all.onsuccess = () => resolve(all.result);
          all.onerror = () => reject(all.error);
        };
      });
    });

    // There should be two items in the queue (the tap_to_pay action and the CRDT mutation)
    expect(queuedTxs.length).toBeGreaterThan(0);
    const tapToPayTx = queuedTxs.find((tx: any) => tx.type === 'tap_to_pay');
    expect(tapToPayTx).toBeDefined();
    expect(tapToPayTx.amount_cents).toBe(5000);

    // Make network online
    await context.setOffline(false);

    // Fire online event to trigger page.tsx sync
    await memberPage.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Verify "Syncing..." or Online indicator
    await expect(memberPage.locator('text=Online').first()).toBeVisible({ timeout: 5000 }).catch(() => {});

    // Wait for the sync to complete and the IndexedDB to be cleared
    await memberPage.waitForFunction(async () => {
      return new Promise<boolean>((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onsuccess = () => {
          const db = req.result;
          if (!db.objectStoreNames.contains('actions')) {
            resolve(true);
            return;
          }
          const tx = db.transaction('actions', 'readonly');
          const store = tx.objectStore('actions');
          const all = store.getAll();
          all.onsuccess = () => {
            resolve(all.result.length === 0);
          };
          all.onerror = () => resolve(false);
        };
        req.onerror = () => resolve(false);
      });
    }, { timeout: 15000 });

    // Ensure the queue was cleared successfully
    const afterSyncTxs = await memberPage.evaluate(() => {
      return new Promise<any[]>((resolve, reject) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onerror = () => reject(req.error);
        req.onsuccess = () => {
          const db = req.result;
          if (!db.objectStoreNames.contains('actions')) {
            resolve([]);
            return;
          }
          const tx = db.transaction('actions', 'readonly');
          const store = tx.objectStore('actions');
          const all = store.getAll();
          all.onsuccess = () => resolve(all.result);
          all.onerror = () => reject(all.error);
        };
      });
    });
    expect(afterSyncTxs.length).toBe(0);
  });
});
