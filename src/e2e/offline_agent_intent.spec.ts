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
    await page.evaluate(() => {
        const action = {
            id: 'intent-test-id-123',
            type: 'agent_intent',
            payload: { action: 'draft_email', recipient: 'customer@example.com', subject: 'Follow up' },
            timestamp: Date.now()
        };
        const request = window.indexedDB.open("OHC_Offline_Queue", 1);
        request.onsuccess = (e) => {
            const db = (e.target as any).result;
            const tx = db.transaction("actions", "readwrite");
            tx.objectStore("actions").put(action);
            tx.oncomplete = () => window.dispatchEvent(new Event("ohc_queue_updated"));
        };
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
            const request = window.indexedDB.open("OHC_Offline_Queue", 1);
            request.onsuccess = (e) => {
                const db = (e.target as any).result;
                try {
                    const tx = db.transaction("actions", "readonly");
                    const req = tx.objectStore("actions").getAll();
                    req.onsuccess = () => resolve(req.result.length === 0);
                    req.onerror = () => resolve(false);
                } catch { resolve(true); }
            };
            request.onerror = () => resolve(true);
        });
    }, { timeout: 15000 });

    const queueLength = await page.evaluate(async () => {
        return new Promise((resolve) => {
            const request = window.indexedDB.open("OHC_Offline_Queue", 1);
            request.onsuccess = (e) => {
                const db = (e.target as any).result;
                try {
                    const tx = db.transaction("actions", "readonly");
                    const req = tx.objectStore("actions").getAll();
                    req.onsuccess = () => resolve(req.result.length);
                    req.onerror = () => resolve(0);
                } catch { resolve(0); }
            };
            request.onerror = () => resolve(0);
        });
    });
    expect(queueLength).toBe(0);

    // The network status indicator should disappear since we are online and queue is empty
    await expect(page.locator('#network-status-indicator')).toHaveClass(/hidden/, { timeout: 5000 });
  });
});
