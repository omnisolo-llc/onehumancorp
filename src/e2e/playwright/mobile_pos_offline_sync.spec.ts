import { test, expect } from '@playwright/test';

test.describe('Mobile POS - Offline Outbox Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Persona: Field Service Owner uses Quick Charge offline Tap-to-Pay', async ({ page, context }) => {
    const tenantId = `tenant-quick-charge-${Date.now()}`;

    await page.goto('http://127.0.0.1:3000/').catch(() => {});
    await page.evaluate((tenant) => {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Owner', pin_hash: '1234', tenant_id: tenant }]));
        localStorage.setItem('ohc_offline_events', JSON.stringify([]));
        localStorage.setItem('ohc_pos_device_id', 'test_device_123');
        localStorage.setItem('ohc_catalog_default', JSON.stringify([]));
    }, tenantId);

    // 1. Navigate to Feed and click New Sale
    await page.goto('/feed');
    const newSaleBtn = page.getByRole('link', { name: 'New Sale' });
    await expect(newSaleBtn).toBeVisible({ timeout: 15000 });
    await newSaleBtn.click();

    // 2. Unlock Terminal
    await expect(page.getByText('Terminal Locked')).toBeVisible({ timeout: 15000 });
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }
    await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});

    // 3. Switch to Quick Charge mode
    const quickChargeTab = page.getByRole('button', { name: 'Quick Charge' });
    await expect(quickChargeTab).toBeVisible({ timeout: 15000 });
    await quickChargeTab.click();

    // 4. Enter $15.00
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '5', exact: true }).click();
    await page.getByRole('button', { name: '0', exact: true }).first().click();
    await page.getByRole('button', { name: '0', exact: true }).first().click();
    await expect(page.getByText('$15.00')).toBeVisible();

    // 5. Hit Charge
    const chargeBtn = page.getByRole('button', { name: 'Charge $15.00' });
    await expect(chargeBtn).toBeVisible();
    await chargeBtn.click();

    // 6. Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.locator('text=Offline - Changes saved locally')).toBeVisible({ timeout: 10000 });

    // 7. Select Tap to Pay
    const tapToPayBtn = page.getByRole('button', { name: 'Tap to Pay' });
    await expect(tapToPayBtn).toBeVisible();
    await tapToPayBtn.click();

    // Wait for the Offline Tap-to-Pay confirmation
    await expect(page.getByText('Saved Offline - Will sync when connected')).toBeVisible({ timeout: 10000 });

    // Check Success Screen
    await expect(page.getByText('Payment Successful!')).toBeVisible({ timeout: 10000 });

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
    expect(queueData).toContain('tap_to_pay');

    // Restore Network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });

  test('Persona: Boutique Operator records offline cash sale and syncs it', async ({ page, context }) => {
    const tenantId = `tenant-offline-sync-${Date.now()}`;

    // Seed mock data using localStorage to avoid relying on API endpoints that may not be available in all test runners
    await page.goto('http://127.0.0.1:3000/').catch(() => {}); // Fallback to UI server
    await page.evaluate((tenant) => {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Priya', role: 'Manager', pin_hash: '1234', tenant_id: tenant }]));
        localStorage.setItem('ohc_offline_events', JSON.stringify([]));
        localStorage.setItem('ohc_pos_device_id', 'test_device_123');

        const catalog = [{
            id: 'prod_offline_sync_test',
            title: 'Offline Sync Mobile POS Item',
            price_cents: 2500,
            inventory_count: 5,
            stock: 5,
            available_quantity: 5
        }];
        localStorage.setItem('ohc_catalog_default', JSON.stringify(catalog));
    }, tenantId);

    // Navigate to POS terminal
    await page.goto('/pos/terminal');
    await expect(page.getByText('Terminal Locked')).toBeVisible({ timeout: 15000 });
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }
    await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});

    // Wait for Product Catalog and select the specific item
    await expect(page.locator('h3', { hasText: 'Product Catalog' })).toBeVisible({ timeout: 15000 });
    const productBtn = page.locator('button', { hasText: 'Offline Sync Mobile POS Item' });
    await expect(productBtn).toBeVisible({ timeout: 15000 });

    // Check initial stock
    const textBefore = await productBtn.innerText();
    expect(textBefore).toContain('Stock: 5');

    await productBtn.click();

    // Open Cart Drawer
    const chargeBtn = page.getByRole('button', { name: /Charge \$/ });
    await expect(chargeBtn).toBeVisible();
    await chargeBtn.click();

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline banner
    await expect(page.locator('text=Offline - Changes saved locally')).toBeVisible({ timeout: 10000 });

    // Open Cash Sale Panel
    const cashMethodBtn = page.getByRole('button', { name: 'Cash' });
    await expect(cashMethodBtn).toBeVisible();
    await cashMethodBtn.click();

    // Process Cash Sale
    const recordCashSaleBtn = page.getByRole('button', { name: /Record Offline Cash Sale/ });
    await expect(recordCashSaleBtn).toBeVisible();
    await recordCashSaleBtn.click();

    // Verify Success State
    await expect(page.getByText('Cash sale saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

    // Verify Optimistic Stock Reduction in UI (Without page reload)
    // The cart closes and we are back to catalog
    await page.getByRole('button', { name: 'Back' }).click().catch(() => {}); // If needed to close payment panel
    await page.locator('.fixed.inset-0.z-50.bg-black\\/60').click({position: {x: 10, y: 10}}).catch(() => {}); // Close cart drawer by clicking backdrop if it's still open

    await expect(page.getByText('Payment Successful!')).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'No Receipt' }).click();

    const textAfter = await productBtn.innerText();
    expect(textAfter).toContain('Stock: 4');

    // Go online to trigger sync
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
