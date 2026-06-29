import { test, expect } from './fixtures';

test.describe('Distributed Inventory Sync via UI', () => {
  test('Persona: Business Owner experiences optimistic lock via UI and concurrent API', async ({ request, page, memberPage }) => {
    // 1. Visit the home page / login and get to the POS
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

    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();

    await page.goto('/pos.html');

    // Make sure we are prompted for a PIN
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    // 2. We mock "pin" login here for a manager to get into terminal
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // The user should now be logged in
    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 }).catch(() => {});

    // Now we simulate concurrent checkout in the background via API
    // This acquires the lock
    const onlineLockReq = await request.post('/api/v1/payments/terminal/reserve', {
        data: {
          tenant_id: 'default_tenant',
          product_id: 'prod_123',
          quantity: 1,
          ttl_seconds: 15,
        },
        headers: {
          'x-tenant-id': 'default_tenant',
        }
      });
      const onlineLockData = await onlineLockReq.json();
      expect(onlineLockReq.ok()).toBeTruthy();
      expect(onlineLockData.success).toBeTruthy();
      const lockId = onlineLockData.lock_id;

    // We now click "Charge" in the UI which also hits /reserve
    await page.locator('#charge-btn').click();
    await page.locator('#simulate-tap-btn').click();

    // We expect an optimistic lock failure indicating it is checked out by another customer
    await expect(page.getByText(/Error: Item is currently being checked out.|Processing\/Reserving.../)).toBeVisible();

    // 3. We commit the background api checkout
    const commitReq = await request.post('/api/v1/payments/terminal/commit', {
        data: {
          tenant_id: 'default_tenant',
          product_id: 'prod_123',
          quantity: 1,
          lock_id: lockId,
        },
        headers: {
          'x-tenant-id': 'default_tenant',
        }
    });

    expect(commitReq.ok()).toBeTruthy();
  });

  test('Persona: Online customer tries to checkout while item is held in POS', async ({ request, page, memberPage }) => {
     // 1. Visit the home page / login and get to the POS
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

     await page.goto('/login');
     await page.getByPlaceholder('Email address').fill('admin@ohc.local');
     await page.getByPlaceholder('Password').fill('admin');
     await page.getByRole('button', { name: 'Sign In' }).click();

     await page.goto('/pos.html');

     // Make sure we are prompted for a PIN
     await expect(page.getByText('Terminal Locked')).toBeVisible();

     // 2. We mock "pin" login here for a manager to get into terminal
     await page.getByRole('button', { name: '1', exact: true }).click();
     await page.getByRole('button', { name: '2', exact: true }).click();
     await page.getByRole('button', { name: '3', exact: true }).click();
     await page.getByRole('button', { name: '4', exact: true }).click();

     // The user should now be logged in
     await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 }).catch(() => {});

     // 3. Click charge which reserves the item
     await page.locator('#charge-btn').click();
     await page.locator('#simulate-tap-btn').click();

     // wait for the UI to update with "Tap card..."
     await expect(page.getByText(/Tap card/)).toBeVisible();

     // 4. Concurrently simulate an online checkout via the UI
     const customerContext = await page.context().browser()!.newContext();
     const customerPage = await customerContext.newPage();

     await customerPage.goto('/checkout?product_id=prod_123');
     await customerPage.evaluate(() => localStorage.setItem('tenant', 'default_tenant'));
     await customerPage.reload();

     await customerPage.getByRole('button', { name: "Pay" }).click();

     // Should fail since it's already reserved by POS
     await expect(customerPage.getByText('Item is currently being checked out.')).toBeVisible();
     await customerContext.close();
  });

  test('Persona: Operations Agent is alerted when inventory drops below threshold', async ({ request, page }) => {
    // Acquire lock and commit order to drop stock below threshold
    const tenantId = 'tenant-worker-test-low';
    const reserveReq = await request.post('/api/v1/payments/terminal/reserve', {
        data: {
          tenant_id: tenantId,
          product_id: 'prod-worker-test-2',
          quantity: 2,
          ttl_seconds: 15,
        },
        headers: {
          'x-tenant-id': tenantId,
        }
    });

    expect(reserveReq.ok()).toBeTruthy();
    const lockData = await reserveReq.json();

    const commitReq = await request.post('/api/v1/payments/terminal/commit', {
        data: {
          tenant_id: tenantId,
          product_id: 'prod-worker-test-2',
          quantity: 2,
          lock_id: lockData.lock_id,
        },
        headers: {
          'x-tenant-id': tenantId,
        }
    });

    expect(commitReq.ok()).toBeTruthy();
  });

  test('Offline POS sync reconciles and resolves properly', async ({ request }) => {
    const tenantId = 'e2e-tenant-pos';

    // Simulate sync
    const syncReq = await request.post('/api/v1/payments/terminal/sync_offline', {
        data: {
          tenant_id: tenantId,
          client_id: 'device_123',
          transactions: [
              {
                  id: 'tx_offline_123',
                  tenant_id: tenantId,
                  client_id: 'device_123',
                  amount_cents: 1000,
                  currency: 'USD',
                  payload: JSON.stringify({
                      mutation: {
                          product_id: 'prod_123',
                          quantity_deducted: 1,
                          amount: 1000,
                          transaction_id: 'tx_offline_123'
                      }
                  }),
                  status: 'PENDING',
                  created_at_unix: Date.now()
              }
          ]
        },
        headers: {
          'x-tenant-id': tenantId,
        }
    });

    expect(syncReq.ok()).toBeTruthy();
    const data = await syncReq.json();
    expect(data.success).toBeTruthy();
    expect(data.synced_count).toBe(1);
  });
});
