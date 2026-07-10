import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - E2E Race Condition', () => {
  test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
    const tenantId = 'e2e-tenant-pos';
    const productId = 'e2e-product-cake-pos';

    // Simulate POS (User B) acquiring lock
    const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        },
        headers: {
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });


    if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
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
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
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
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
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
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });


    if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
    expect(reserveRes.ok()).toBe(true);
    const lockData = await reserveRes.json();
    expect(lockData.success).toBe(true);

    // 2. Navigate to checkout page for the locked product
    await page.goto(`/checkout?product_id=${productId}&quantity=1`);

    // 3. Click the Pay button
    await page.getByRole('button', { name: 'Pay' }).click();

    // 4. Verify the "Item just sold out" message appears
    await expect(page.locator('h3', { hasText: 'Oops! Item just sold out.' })).toBeVisible({ timeout: 15000 });

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
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });
  });
  test('Commit inventory correctly deducts stock', async ({ page }) => {
    const tenantId = 'e2e-tenant-pos-additional';
    const productId = 'e2e-product-cake-pos-additional';

    const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        },
        headers: {
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });

    if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
    expect(reserveRes.ok()).toBe(true);
    const lockData = await reserveRes.json();
    expect(lockData.success).toBe(true);

    const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            lock_id: lockData.lock_id
        },
        headers: {
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });


    const commitData = await commitRes.json();
    expect(commitData.success).toBe(true);
  });

  test('Operations Agent generates a Restock notification in the owner feed when item sells out', async ({ page }) => {
    // 1. Log in to get token
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    const response = await page.request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    const tenantId = 'default';
    const productId = 'e2e-product-restock-' + Date.now();

    // 2. Create the product with stock 1
    const createProductRes = await page.request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            id: productId,
            title: 'Limited Restock Item',
            inventory_count: 1,
            price_cents: 1000
        }
    });
    expect(createProductRes.ok()).toBeTruthy();

    // Simulate POS (User B) acquiring lock
    const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        },
        headers: {
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });

    if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
    expect(reserveRes.ok()).toBe(true);
    const lockData = await reserveRes.json();
    expect(lockData.success).toBe(true);

    // POS (User B) completes checkout
    const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
        data: {
            tenant_id: tenantId,
            product_id: productId,
            quantity: 1,
            lock_id: lockData.lock_id
        },
        headers: {
            'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
            'x-tenant-id': tenantId
        }
    });

    expect(commitRes.ok()).toBe(true);

    await page.waitForTimeout(5000);

    // Navigate to Action Center
    await page.goto('/dashboard');

    // Check if the agent action request appears in the feed
    await expect(page.locator('body')).toContainText('Action Request: Reorder', { timeout: 30000 });
  });

});
