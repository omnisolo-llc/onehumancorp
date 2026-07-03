import { test, expect } from '@playwright/test';

test.describe('Offline Mobile Sync & Tap-to-Pay Architecture', () => {
  test('should process an offline payment and sync it when online', async ({ page, context }) => {
    // Navigate to terminal
    await page.goto('/pos.html');

    // Simulate terminal setup logic, we will just click the new order and pretend it's connected
    // This is simplified as the UI needs a pin to unlock.
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();

    // Clock in
    await page.getByText('Clock In').click();

    // Connect to a mocked reader
    await page.getByText('Discover Readers').click();
    await page.waitForTimeout(500);
    const connectButton = page.getByText('Connect').first();
    if (await connectButton.isVisible()) {
        await connectButton.click();
    }

    // Go offline
    await context.setOffline(true);

    // Process payment
    await page.locator('button', { hasText: 'Charge $' }).first().click();

    // Should show offline success
    await expect(page.getByText('Tap-to-Pay saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

    // Verify it's in the queue (IndexedDB)
    const queueData = await page.evaluate(async () => {
        return new Promise<string>((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) return resolve('[]');
                const tx = db.transaction('actions', 'readonly');
                const reqAll = tx.objectStore('actions').getAll();
                reqAll.onsuccess = () => resolve(JSON.stringify(reqAll.result));
            };
            req.onerror = () => resolve('[]');
        });
    });
    expect(queueData).toContain('tap_to_pay');

    // Wait for sync to happen. Without network mocking, it goes through the actual api endpoints.
    // Go online
    await context.setOffline(false);

    await page.waitForTimeout(2000);

    // Verify queue is empty
    const updatedQueueData = await page.evaluate(async () => {
        return new Promise<string>((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) return resolve('[]');
                const tx = db.transaction('actions', 'readonly');
                const reqAll = tx.objectStore('actions').getAll();
                reqAll.onsuccess = () => resolve(JSON.stringify(reqAll.result));
            };
            req.onerror = () => resolve('[]');
        });
    });
    expect(updatedQueueData).toBe('[]');
  });

  test('should trigger Operations Agent reconciliation card on negative inventory conflict', async ({ page, context }) => {
    // Navigate to terminal
    await page.goto('/pos.html');

    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();

    await page.getByText('Clock In').click();

    await page.getByText('Discover Readers').click();
    await page.waitForTimeout(500);
    const connectButton = page.getByText('Connect').first();
    if (await connectButton.isVisible()) {
        await connectButton.click();
    }

    // Go offline
    await context.setOffline(true);

    // Click charge to process an offline payment
    await page.locator('button', { hasText: 'Charge $' }).first().click();
    await expect(page.getByText('Tap-to-Pay saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

    // Modify the quantity in IndexedDB to force a conflict since the UI doesn't allow changing quantity
    await page.evaluate(async () => {
        return new Promise((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) return resolve(true);
                const tx = db.transaction('actions', 'readwrite');
                const store = tx.objectStore('actions');
                const reqAll = store.getAll();
                reqAll.onsuccess = () => {
                    const queue = reqAll.result;
                    if (queue.length > 0) {
                        queue[0].quantity = 100; // Force conflict
                        queue[0].product_id = 'prod_123';
                        store.put(queue[0]);
                    }
                };
                tx.oncomplete = () => resolve(true);
            };
        });
    });

    // Go online to trigger sync
    await context.setOffline(false);

    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    await page.waitForTimeout(8000);

    const updatedQueueData2 = await page.evaluate(async () => {
        return new Promise<string>((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) return resolve('[]');
                const tx = db.transaction('actions', 'readonly');
                const reqAll = tx.objectStore('actions').getAll();
                reqAll.onsuccess = () => resolve(JSON.stringify(reqAll.result));
            };
            req.onerror = () => resolve('[]');
        });
    });
    expect(updatedQueueData2).toBe('[]');

    // Navigate to Dashboard/Agent Feed
    await page.goto('/dashboard');

    // Look for the action card generated by Operations Agent
    await expect(page.getByText(/We oversold the item prod_123 by /)).toBeVisible({ timeout: 15000 });
  });
});
