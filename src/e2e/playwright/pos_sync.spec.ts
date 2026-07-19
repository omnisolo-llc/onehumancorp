import { test, expect } from '@playwright/test';

test.describe('Mobile POS - Offline POS Sync API', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Owner synchronizes offline transactions via API and sees inventory update', async ({ page, request }) => {
    // Navigate to feed and login to get a tenant environment
    const tenantId = `tenant-sync-${Date.now()}`;
    await page.goto('http://127.0.0.1:3000/').catch(() => {});
    await page.evaluate((tenant) => {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Maya', role: 'Owner', pin_hash: '1234', tenant_id: tenant }]));
    }, tenantId);

    // Seed some inventory for this tenant directly
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/x`;

    // Create a product to sync against using the internal DB API or simply rely on the default sync logic
    // which gracefully handles missing items by creating an agent_intent / failing outbox gracefully.
    // But we will test the endpoint idempotency and success logic.

    const txId = `tx-sync-${Date.now()}`;
    const payload = {
        transactions: [
            {
                transaction_id: txId,
                product_id: "test-product-123",
                quantity_deducted: 1,
                amount_cents: 2500,
                currency: "USD",
                client_mutation_id: `mut-sync-${Date.now()}`
            }
        ]
    };

    // First request should succeed and enqueue job
    const response = await request.post('http://127.0.0.1:3000/api/v1/pos/sync', {
        headers: {
            'x-spiffe-id': spiffeId
        },
        data: payload
    });

    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.failed_count).toBe(0);

    // Second request should succeed but do nothing due to deduplication (ON CONFLICT DO NOTHING)
    const dupResponse = await request.post('http://127.0.0.1:3000/api/v1/pos/sync', {
        headers: {
            'x-spiffe-id': spiffeId
        },
        data: payload
    });
    expect(dupResponse.status()).toBe(200);
    const dupBody = await dupResponse.json();
    expect(dupBody.success).toBe(true);
    expect(dupBody.failed_count).toBe(0);

    // Now verify the UI interaction aspect.
    // The prompt explicitly requires "Create an E2E Playwright test simulating an owner using a mocked offline-to-online sync flow, ensuring inventory updates correctly on the dashboard."

    // 1. Visit the inventory dashboard
    await page.goto('/catalog');

    // Wait for the UI to load
    await page.waitForLoadState('networkidle');

    // Since we didn't mock a product creation UI flow, let's use the UI to create a product, THEN sync it offline, THEN verify it.
    await page.getByRole('button', { name: 'Add Product' }).click();
    await page.getByLabel('Name').fill('Offline Sync Cake');
    await page.getByLabel('Price').fill('25');
    await page.getByLabel('Stock').fill('10');
    await page.getByRole('button', { name: 'Save' }).click();

    // Wait for save to complete
    await expect(page.getByText('Offline Sync Cake')).toBeVisible();

    // Fetch products to find the created one to sync against
    const inventoryRes = await request.get('/api/v1/pos/inventory', { headers: { 'x-spiffe-id': spiffeId } });
    const inventoryBody = await inventoryRes.json();
    const testProduct = inventoryBody.inventory.find(p => p.name === 'Offline Sync Cake');

    expect(testProduct).toBeDefined();

    const newTxId = `tx-sync-ui-${Date.now()}`;
    const newPayload = {
        transactions: [
            {
                transaction_id: newTxId,
                product_id: testProduct.id,
                quantity_deducted: 3,
                amount_cents: 7500,
                currency: "USD",
                client_mutation_id: `mut-sync-ui-${Date.now()}`
            }
        ]
    };

    // Trigger sync
    const uiSyncRes = await request.post('/api/v1/pos/sync', {
        headers: {
            'x-spiffe-id': spiffeId
        },
        data: newPayload
    });
    expect(uiSyncRes.status()).toBe(200);

    // Wait for backend async worker to run.
    await page.waitForTimeout(3000);

    // Reload the catalog page to see the updated inventory
    await page.reload();
    await page.waitForLoadState('networkidle');

    // Original stock was 10, deducted 3, should be 7
    // Verify the UI shows the new stock
    await expect(page.getByText('Offline Sync Cake').locator('..').getByText('7')).toBeVisible();
  });
});
