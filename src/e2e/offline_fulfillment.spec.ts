import { test, expect } from '@playwright/test';

test.describe('Fulfillment Hub - Offline Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Persona: Operator processes fulfillment actions offline', async ({ page, context, request }) => {
    // 1. Get token
    const response = await request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    const { token } = await response.json();

    const tenantId = `tenant-fulfillment-${Date.now()}`;

    // 2. Set up initial session
    await page.goto('/login');
    await page.evaluate((tenant) => {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    }, tenantId);

    // 3. Navigate to fulfillment hub
    await page.goto('/fulfillment-hub');

    // 4. Wait for initial orders to load
    // The backend router returns standard dummy orders initially so we should expect "To Pack" and "Awaiting Pickup" sections
    await expect(page.getByRole('heading', { name: 'To Pack' })).toBeVisible({ timeout: 15000 });
    const markReadyBtn = page.getByRole('button', { name: 'Mark Ready' }).first();
    await expect(markReadyBtn).toBeVisible({ timeout: 15000 });

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline banner
    await expect(page.locator('text=Offline Mode')).toBeVisible({ timeout: 10000 });

    // Perform action
    await markReadyBtn.click();

    // UI should optimistically update (Wait for "Ready for" badge in Awaiting pickup section)
    await expect(page.getByText(/Ready for/)).toBeVisible({ timeout: 10000 });

    // Verify it's in the IndexedDB offline queue
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

    expect(queueData).toContain('fulfillment_action');

    // Go online to trigger sync
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Ensure the offline queue clears
    await page.waitForTimeout(5000);
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
});
