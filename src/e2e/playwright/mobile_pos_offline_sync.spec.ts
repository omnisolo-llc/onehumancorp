import { test, expect } from '@playwright/test';

test.describe('Mobile POS - Offline Outbox Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Persona: Boutique Operator records offline cash sale and syncs it', async ({ page, context, request }) => {
    const tenantId = `tenant-offline-sync-${Date.now()}`;
    const productId = `prod-offline-sync-${Date.now()}`;

    // 1. Seed the database with a user, tenant, and product
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('pos_user_offline_id', 'pos_offline@example.com', 'POS Offline User', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'POS Offline Store', 'pos_offline@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO products (id, tenant_id, title, description, price_cents, inventory_count, available_quantity)
          VALUES ('${productId}', '${tenantId}', 'Offline Sync Mobile POS Item', 'Offline Item', 2500, 5, 5)
          ON CONFLICT DO NOTHING;
        `
      }
    });

    // We also need to seed staff for this tenant so the POS terminal allows login
    await page.goto(`/login?test_email=pos_offline@example.com`);
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
    await expect(page.locator('text=Offline Mode')).toBeVisible({ timeout: 10000 });

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

    // Verify it's in the IndexedDB offline queue
    const queueData = await page.evaluate(async () => {
        return new Promise((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = e.target.result;
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
        return new Promise((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = e.target.result;
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
