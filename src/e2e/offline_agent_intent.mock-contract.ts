import { test, expect } from './fixtures';

test.describe('Offline Agent Intent Sync', () => {
  test('should queue agent intent mutations locally when offline and sync when online', async ({ page, context }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard.html');

    // Set network to offline
    await context.setOffline(true);

    // Evaluate to simulate the offline environment trigger
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // The network status indicator should show offline
    await expect(page.locator('#network-status-indicator').first()).toBeVisible();
    await expect(page.locator('#network-status-text').first()).toHaveText('Working Offline');

    // Enqueue an agent intent mutation into IndexedDB
    await page.evaluate(async () => {
        await new Promise((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) {
                    resolve(true);
                    return;
                }
                const tx = db.transaction('actions', 'readwrite');
                tx.objectStore('actions').put({
                    id: 'intent-test-id-123',
                    type: 'agent_intent',
                    payload: { action: 'draft_email', recipient: 'customer@example.com', subject: 'Follow up' },
                    timestamp: new Date().getTime()
                });
                tx.oncomplete = () => resolve(true);
            };
            req.onerror = () => resolve(true);
        });
        // Trigger queue update
        window.dispatchEvent(new Event('ohc_queue_updated'));
    });

    // Verify queue indicator shows items pending
    await expect(page.locator('#queue-dashboard')).toBeVisible();
    await expect(page.locator('#queue-dashboard')).toContainText('1 Items Pending Sync');

    // Set network to online
    await context.setOffline(false);

    // Trigger online event to allow the application to naturally attempt synchronization.
    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    // Wait for the sync to complete and the queue to be cleared
    await page.waitForFunction(async () => {
        return new Promise((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) return resolve(true);
                const tx = db.transaction('actions', 'readonly');
                const reqAll = tx.objectStore('actions').getAll();
                reqAll.onsuccess = () => resolve(reqAll.result.length === 0);
            };
            req.onerror = () => resolve(false);
        });
    }, { timeout: 15000 });

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
    expect(queueData).toBe('[]');

    // The network status indicator should disappear since we are online and queue is empty
    await expect(page.locator('#network-status-indicator')).toHaveClass(/hidden/, { timeout: 5000 });
  });
});
