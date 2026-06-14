import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - E2E Race Condition', () => {
  test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
    const tenantId = 'e2e-tenant';
    const productId = 'e2e-product-cake';

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

  test('Online checkout UI shows Item just sold out when POS locks item', async ({ page }) => {
    const tenantId = 'e2e-tenant';
    const productId = 'e2e-product-cake';

    // 1. Setup tenant info in local storage for checkout page
    await page.goto('/checkout');
    await page.evaluate((tenant) => {
      localStorage.setItem('tenant', tenant);
      localStorage.setItem('customer_id', 'e2e-customer');
    }, tenantId);

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

    // 2. Navigate to checkout page for the locked product
    await page.goto(`/checkout?product_id=${productId}&quantity=1`);

    // 3. Click the Pay button
    await page.getByRole('button', { name: 'Pay' }).click();

    // 4. Verify the "Item just sold out" message appears
    await expect(page.getByText('Item just sold out.')).toBeVisible();

    // Cleanup: Release lock so it doesn't affect other tests if they run concurrently
    // (Actually the lock will expire in 15 seconds, but let's release it cleanly)
    await page.request.post('/api/v1/payments/terminal/commit', {
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
  });
});
