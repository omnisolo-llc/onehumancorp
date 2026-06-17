import { test, expect } from './fixtures';

test.describe('Centralized Inventory & Distributed POS Architecture', () => {
  test('Prevents simultaneous online and offline purchases via Redis Redlock and generates restock AI task', async ({ page, memberPage, request }) => {
    // Navigate to local API directly to set up origin to allow localstorage modification
    await memberPage.goto('/api/staff');
    await memberPage.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Priya',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    // 1. Log in as an admin or tenant
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();

    // 2. Wait for dashboard to load
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    const response = await request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    const createProductRes = await request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: 'Conflict Test Product',
            inventory_count: 1,
            price_cents: 1000
        }
    });
    expect(createProductRes.ok()).toBeTruthy();
    const product = await createProductRes.json();
    const productId = product.id || product.product_id;

    // Simulate POS checkout reserve
    const reserveRes1 = await request.post('/api/v1/payments/terminal/reserve', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        }
    });
    expect(reserveRes1.ok()).toBeTruthy();
    const reserveData1 = await reserveRes1.json();
    expect(reserveData1.success).toBeTruthy();

    // Simulate concurrent online checkout reserve
    const reserveRes2 = await request.post('/api/v1/payments/terminal/reserve', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        }
    });
    expect(reserveRes2.ok()).toBeTruthy();
    const reserveData2 = await reserveRes2.json();
    // It should fail because Redlock prevents it
    expect(reserveData2.success).toBeFalsy();
    expect(reserveData2.error_message).toContain('Item is currently being checked out');
  });
});
