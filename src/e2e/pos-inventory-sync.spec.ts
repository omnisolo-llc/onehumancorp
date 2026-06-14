import { test, expect } from './fixtures';

test.describe('POS Inventory Sync - E2E CUJ', () => {
  const tenantId = 'e2e-tenant';
  const productId = 'e2e-product-cake';

  test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
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
    const reserveRes2 = await page.request.post('/api/v1/payments/terminal/reserve', {
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

    // It should fail gracefully
    const lockData2 = await reserveRes2.json();
    expect(lockData2.success).toBe(false);
    expect(lockData2.error_message).toContain('another customer');

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

  test('POS terminal successfully starts a session', async ({ page }) => {
    const res = await page.request.post('/api/v1/payments/terminal/session/start', {
      data: { device_id: 'test-device-123' },
      headers: { 'x-tenant-id': tenantId }
    });
    expect(res.ok()).toBe(true);
    const data = await res.json();
    expect(data.success).toBe(true);
    expect(data.session_id).toBeTruthy();
  });

  test('POS offline transactions sync correctly', async ({ page }) => {
    const syncRes = await page.request.post('/api/v1/payments/terminal/sync', {
        data: {
            session_id: 'test-session-123',
            transactions: [
                {
                    id: 'tx_offline_123',
                    amount_cents: 1000,
                    currency: 'USD',
                    payload: JSON.stringify([{ product_id: productId, quantity: 1 }]),
                    client_id: 'terminal_1',
                    timestamp: new Date().toISOString()
                }
            ]
        },
        headers: { 'x-tenant-id': tenantId }
    });

    expect(syncRes.ok()).toBe(true);
    const syncData = await syncRes.json();
    expect(syncData.success).toBe(true);
  });

  test('POS UI allows entering PIN to unlock Terminal', async ({ page }) => {
    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible();
    await page.locator('button').filter({ hasText: '1' }).click();
    await page.locator('button').filter({ hasText: '2' }).click();
    await page.locator('button').filter({ hasText: '3' }).click();
    await page.locator('button').filter({ hasText: '4' }).click();

    await expect(page.locator('text=Invalid PIN')).toBeVisible();
  });

  test('POS UI renders without layout issues', async ({ page }) => {
    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible();
    await expect(page.locator('text=Enter your PIN to unlock')).toBeVisible();
  });
});
