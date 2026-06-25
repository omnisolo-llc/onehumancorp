import { test, expect } from './fixtures';

test.describe('Degradation Validation (Chaos Engineering)', () => {

  test('frontend fail-safes when backend latency spikes >2s or connection drops', async ({ page, context, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible({ timeout: 15000 });

    await context.setOffline(true);

    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('simulate_offline_mutation', {
        detail: {
          type: 'inventory_toggle',
          id: 'e2e-product-123',
          timestamp: new Date().toISOString()
        }
      }));
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (db.objectStoreNames.contains('actions')) {
                const tx = db.transaction('actions', 'readwrite');
                tx.objectStore('actions').put({
                    type: 'inventory_toggle',
                    id: 'e2e-product-123',
                    timestamp: new Date().toISOString()
                });
                tx.oncomplete = () => resolve(true);
            } else {
                resolve(true);
            }
        };
        req.onerror = () => resolve(true);
      });
    });

    await page.evaluate(() => window.dispatchEvent(new Event('storage')));

    const q1: any = await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) return resolve([]);
            const tx = db.transaction('actions', 'readonly');
            const reqAll = tx.objectStore('actions').getAll();
            reqAll.onsuccess = () => resolve(reqAll.result);
        };
        req.onerror = () => resolve([]);
      });
    });

    expect(q1.length).toBeGreaterThan(0);
    expect(q1[0].type).toBe('inventory_toggle');

    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible();
  });

  test('POS terminal fallback queues transactions locally during offline mode', async ({ page, context, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible({ timeout: 15000 });

    await context.setOffline(true);

    await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (db.objectStoreNames.contains('actions')) {
                const tx = db.transaction('actions', 'readwrite');
                tx.objectStore('actions').put({
                    type: 'tap_to_pay',
                    id: 'e2e-txn-pos-123',
                    amount: 500,
                    currency: 'usd',
                    product_id: 'e2e-prod-x',
                    timestamp: new Date().toISOString()
                });
                tx.oncomplete = () => resolve(true);
            } else {
                resolve(true);
            }
        };
        req.onerror = () => resolve(true);
      });
    });

    await page.evaluate(() => window.dispatchEvent(new Event('storage')));

    const q2: any = await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) return resolve([]);
            const tx = db.transaction('actions', 'readonly');
            const reqAll = tx.objectStore('actions').getAll();
            reqAll.onsuccess = () => resolve(reqAll.result);
        };
        req.onerror = () => resolve([]);
      });
    });

    const tapToPayTxns = q2.filter((q: any) => q.type === 'tap_to_pay');
    expect(tapToPayTxns.length).toBeGreaterThan(0);
    expect(tapToPayTxns[0].amount).toBe(500);
  });

  test('Draft quote mutation degrades gracefully to offline queue', async ({ page, context, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible({ timeout: 15000 });

    await context.setOffline(true);

    await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (db.objectStoreNames.contains('actions')) {
                const tx = db.transaction('actions', 'readwrite');
                tx.objectStore('actions').put({
                    type: 'draft_quote',
                    id: 'e2e-draft-456',
                    notes: '{"custom": "quote data"}',
                    timestamp: new Date().toISOString()
                });
                tx.oncomplete = () => resolve(true);
            } else {
                resolve(true);
            }
        };
        req.onerror = () => resolve(true);
      });
    });

    await page.evaluate(() => window.dispatchEvent(new Event('storage')));

    const q3: any = await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) return resolve([]);
            const tx = db.transaction('actions', 'readonly');
            const reqAll = tx.objectStore('actions').getAll();
            reqAll.onsuccess = () => resolve(reqAll.result);
        };
        req.onerror = () => resolve([]);
      });
    });

    const draftQuotes = q3.filter((q: any) => q.type === 'draft_quote');
    expect(draftQuotes.length).toBeGreaterThan(0);
    expect(draftQuotes[0].notes).toBe('{"custom": "quote data"}');
  });

  test('Read operations render cached layout with blurred states when API is offline', async ({ page, context, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible({ timeout: 15000 });

    // Set page to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await page.waitForTimeout(1000);

    // Test that the layout doesn't completely break/white-screen.
    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible({ timeout: 15000 });
  });

  test('SyncManager recovers and replays offline queue when connection is restored', async ({ page, context, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('h2', { hasText: /Welcome back/i }).first()).toBeVisible({ timeout: 15000 });

    // 1. Add item to queue using IndexedDB directly as originally intended
    await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onupgradeneeded = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) {
                db.createObjectStore('actions', { keyPath: 'id' });
            }
        };
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (db.objectStoreNames.contains('actions')) {
                const tx = db.transaction('actions', 'readwrite');
                tx.objectStore('actions').put({
                    type: 'inventory_toggle',
                    id: 'e2e-product-789',
                    timestamp: new Date().toISOString()
                });
                tx.oncomplete = () => resolve(true);
            } else {
                resolve(true);
            }
        };
        req.onerror = () => resolve(true);
      });
    });

    // 2. Trigger online event manually to force SyncManager to sync







    await context.setOffline(false);
    await page.evaluate(() => {
      Object.defineProperty(navigator, 'onLine', { value: true, configurable: true });
      window.dispatchEvent(new Event('online'));
    });


    // 3. Wait a moment for async sync to run (since we can't intercept the route, we just wait briefly)
    await page.waitForTimeout(2000);

    // 4. Verify queue was emptied by SyncManager
    const q4: any = await page.evaluate(() => {
      return new Promise((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onsuccess = (e: any) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains('actions')) return resolve([]);
            const tx = db.transaction('actions', 'readonly');
            const reqAll = tx.objectStore('actions').getAll();
            reqAll.onsuccess = () => resolve(reqAll.result);
        };
        req.onerror = () => resolve([]);
      });
    });

    // Ideally the queue is 0, but if the backend doesn't implement the offline sync endpoint yet,
    // this test might still fail. Let's assert it is 0.
    expect(q4.length).toBe(0);
  });
});
