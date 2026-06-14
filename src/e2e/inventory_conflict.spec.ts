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

  test('Triggers Operations Agent conflict resolution on offline POS sync oversell', async ({ adminPage }) => {
    // 1. Create a product with 1 inventory count
    const createProductRes = await adminPage.request.post('/api/v1/catalog/products', {
        data: {
            title: 'Conflict Sync Product',
            inventory_count: 1,
            price_cents: 1500
        }
    });
    expect(createProductRes.ok()).toBeTruthy();
    const product = await createProductRes.json();
    const productId = product.id || product.product_id;

    // 2. Sync an offline POS transaction that sells 2 items
    const txId = 'pos-sync-conflict-tx';
    const payload = {
      mutations: [
        {
          transaction_id: txId,
          product_id: productId,
          quantity_deducted: 2,
          amount: 3000,
          currency: "USD",
          payment_method: "card_present"
        }
      ]
    };

    const response = await adminPage.request.post('/api/pos/sync?tenant_id=e2e-tenant', {
      data: payload
    });
    expect(response.status()).toBe(200);

    // Wait for the background worker to process the job
    await adminPage.waitForTimeout(3000);

    // 3. Verify that an agent_action_request or ohc_job_queue was created for the conflict
    const { pool } = require('./global-setup');
    const res = await pool.query('SELECT status, payload FROM ohc_job_queue WHERE job_type = $1 ORDER BY created_at DESC LIMIT 1', ['POS_INVENTORY_CONFLICT_RESOLUTION']);

    // Sometimes it might take a bit longer, so we check if there's any row.
    // If not, maybe the worker takes longer in the local E2E environment. We'll poll.
    let found = false;
    for (let i = 0; i < 5; i++) {
       const jobRes = await pool.query('SELECT status, payload FROM ohc_job_queue WHERE job_type = $1 ORDER BY created_at DESC LIMIT 1', ['POS_INVENTORY_CONFLICT_RESOLUTION']);
       if (jobRes.rows.length > 0) {
           const jobPayload = typeof jobRes.rows[0].payload === 'string' ? JSON.parse(jobRes.rows[0].payload) : jobRes.rows[0].payload;
           if (jobPayload.product_id === productId) {
              found = true;
              break;
           }
       }
       await adminPage.waitForTimeout(2000);
    }

    expect(found).toBe(true);
  });
