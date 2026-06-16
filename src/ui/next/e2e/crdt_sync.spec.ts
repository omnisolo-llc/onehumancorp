import { test, expect } from '@playwright/test';

test.describe('CRDT Offline Sync Flow', () => {
  test('should queue CRDT delta and sync when online', async ({ page, context }) => {
    await page.goto('/dashboard');

    // Evaluate in page context to mock SyncManager and queue an action
    await page.evaluate(async () => {
      // simulate offline
      window.dispatchEvent(new Event('offline'));

      const action = {
        id: `tx-crdt-e2e-${Date.now()}`,
        type: 'crdt_delta',
        entity_id: 'e2e-product-crdt',
        quantity_deducted: 2,
        payload: {
          quantity_deducted: 2
        }
      };

      // @ts-ignore
      if (window.__ohc_enqueue) {
         // @ts-ignore
         await window.__ohc_enqueue(action);
      } else {
         const DB_NAME = "OHC_Offline_Queue";
         const STORE_NAME = "actions";
         const DB_VERSION = 1;
         const request = window.indexedDB.open(DB_NAME, DB_VERSION);
         request.onsuccess = (e) => {
             // @ts-ignore
             const db = e.target.result;
             const tx = db.transaction([STORE_NAME], 'readwrite');
             const store = tx.objectStore(STORE_NAME);
             store.put({...action, timestamp: Date.now()});
             window.dispatchEvent(new Event('ohc_queue_updated'));
         };
      }
    });

    // Wait for the indicator to appear
    const indicator = page.locator('text=Offline - Saving Locally');
    await expect(indicator).toBeVisible({ timeout: 5000 });

    // Simulate going online
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // The indicator should disappear after syncing
    await expect(indicator).toBeHidden({ timeout: 10000 });
  });
});
