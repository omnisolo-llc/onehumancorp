import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - E2E Race Condition', () => {
  test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
    const tenantId = 'e2e-tenant';
    const productId = 'e2e-product-cake';
    const spiffeId = `spiffe://${tenantId}/user/default`;

    // Simulate POS (User B) acquiring lock
    const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        },
        headers: {
            'x-spiffe-id': spiffeId
        }
    });

    expect(reserveRes.ok()).toBe(true);
    const lockData = await reserveRes.json();
    expect(lockData.success).toBe(true);

    // Create cart for Online User (User A)
    const cartRes = await page.request.post('/api/v1/cart', {
        data: {
            customer_id: 'e2e-customer',
            channel: 'online',
            currency: 'usd'
        },
        headers: {
            'x-spiffe-id': spiffeId
        }
    });
    expect(cartRes.ok()).toBe(true);
    const cartData = await cartRes.json();
    const cartId = cartData.id;

    // Simulate Online User (User A) attempting checkout (adding item to cart) for the same item
    const reserveRes2 = await page.request.post(`/api/v1/cart/${cartId}/items`, {
        data: {
            product_id: productId,
            quantity: 1,
            unit_price_cents: 1000
        },
        headers: {
            'x-spiffe-id': spiffeId
        }
    });

    // It should fail gracefully
    const lockData2 = await reserveRes2.json();
    expect(reserveRes2.ok()).toBe(false);
    expect(lockData2.error).toContain('another customer');

    // POS (User B) completes checkout
    const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            lock_id: lockData.lock_id
        },
        headers: {
            'x-spiffe-id': spiffeId
        }
    });

    expect(commitRes.ok()).toBe(true);
  });
});
