import { test, expect } from '@playwright/test';

test.describe('Mobile POS - Offline Outbox Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Persona: Boutique Operator records offline cash sale and syncs it', async ({ page, context, request }) => {
    // 1. Get token
    const response = await request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    const { token } = await response.json();

    const tenantId = `tenant-offline-sync-${Date.now()}`;
    const productId = `prod-offline-sync-${Date.now()}`;

    // 2. Create the limited stock product via API
    await request.post('/api/v1/catalog/products', {
        headers: {
            'Authorization': `Bearer ${token}`,
            'x-tenant-id': tenantId
        },
        data: {
            id: productId,
            title: 'Offline Sync Mobile POS Item',
            inventory_count: 5,
            price_cents: 2500
        }
    });

    // We also need to seed staff for this tenant so the POS terminal allows login
    await page.goto('/login');
    await page.evaluate((tenant) => {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{
            id: 'staff_1',
            name: 'Priya',
            role: 'Manager',
            pin_hash: '1234',
            tenant_id: tenant
        }]));
        localStorage.setItem('ohc_offline_events', JSON.stringify([]));
        localStorage.setItem('ohc_pos_device_id', 'test_device_123');
    }, tenantId);

    // 3. Navigate to POS terminal
    await page.goto('/pos/terminal');
    await expect(page.getByText('Terminal Locked')).toBeVisible({ timeout: 15000 });
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }
    await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});

    // 4. Wait for Product Catalog and select the specific item
    await expect(page.locator('h3', { hasText: 'Product Catalog' })).toBeVisible({ timeout: 15000 });
    const productBtn = page.locator('button', { hasText: 'Offline Sync Mobile POS Item' });
    await expect(productBtn).toBeVisible({ timeout: 15000 });
    await productBtn.click();

    // 5. Open Cart Drawer and verify offline state
    const chargeBtn = page.getByRole('button', { name: /Charge \$/ });
    await expect(chargeBtn).toBeVisible();
    await chargeBtn.click();

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline banner
    await expect(page.locator('text=Offline - Changes saved locally')).toBeVisible({ timeout: 10000 });

    // Process Cash Sale
    const recordCashSaleBtn = page.getByRole('button', { name: /Record Offline Cash Sale \$/ });
    await expect(recordCashSaleBtn).toBeVisible();
    await recordCashSaleBtn.click();

    await expect(page.getByText('Cash sale saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

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

    expect(queueData).toContain('cash_sale');

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
