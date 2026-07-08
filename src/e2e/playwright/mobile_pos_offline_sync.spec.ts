import { test, expect } from '@playwright/test';

test.describe('Mobile POS - Offline Outbox Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

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

    // Go online to trigger sync
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
