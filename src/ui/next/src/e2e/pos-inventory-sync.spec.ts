import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - E2E Race Condition', () => {
  test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
    const tenantId = 'e2e-tenant-pos';
    const productId = 'prod_123';

    // Simulate POS (User B) acquiring lock
    const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        },
        headers: {
            'x-tenant-id': tenantId
        }
    });

    expect(reserveRes.ok()).toBe(true);
    const lockData = await reserveRes.json();
    expect(lockData.success).toBe(true);

    // Simulate Online User (User A) attempting checkout for the same item
    const reserveRes2 = await page.request.post('/api/v1/booking/conversational_checkout', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            customer_id: 'cust-1',
            amount_cents: 1000
        },
        headers: {
            'x-tenant-id': tenantId
        }
    });

    // It should fail gracefully
    const lockData2 = await reserveRes2.json();
    // Wait, the online checkout will fail but what is the response?
    // In conversational_checkout, it returns 429 Resource Exhausted if lock fails.
    expect(reserveRes2.status()).toBe(429);
    expect(lockData2.message).toContain('another customer');

    // POS (User B) completes checkout
    const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            lock_id: lockData.lock_id
        },
        headers: {
            'x-tenant-id': tenantId
        }
    });

    expect(commitRes.ok()).toBe(true);
  });

  test('Online checkout soft lock prevents POS terminal overselling', async ({ page }) => {
    const tenantId = 'e2e-tenant-pos';
    const productId = 'prod_123';

    // Simulate Online User (User A) starting a conversational checkout
    const onlineRes = await page.request.post('/api/v1/booking/conversational_checkout', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            customer_id: 'cust-1',
            amount_cents: 1000
        },
        headers: {
            'x-tenant-id': tenantId
        }
    });

    expect(onlineRes.ok()).toBe(true);
    const onlineData = await onlineRes.json();
    expect(onlineData.session_id).toBeDefined();

    // Simulate POS (User B) attempting to reserve the item
    const posRes = await page.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 2, // Assuming capacity is only 2, and 1 is taken
            ttl_seconds: 15
        },
        headers: {
            'x-tenant-id': tenantId
        }
    });

    const posData = await posRes.json();
    expect(posData.success).toBe(false);
    expect(posData.error_message).toContain('Insufficient inventory');
  });
});
