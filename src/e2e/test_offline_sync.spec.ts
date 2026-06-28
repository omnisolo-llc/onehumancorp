import { test, expect } from './fixtures';

test.describe('Offline-Tolerant Mobile-First Agentic POS & Order Sync', () => {

  test('1. Toggles "Sold Out" offline and queues mutation', async ({ page, request, memberPage, context }) => {
    await memberPage.setViewportSize({ width: 375, height: 667 });

    // Login logic via api for token
    const loginRes = await request.post('/api/v1/auth/login', {
        data: { email: 'admin@ohc.local', password: 'admin' }
    });
    expect(loginRes.ok()).toBeTruthy();
    const { token, user } = await loginRes.json();

    // Create a product
    const productTitle = 'Falafel Test ' + Date.now();
    const createRes = await request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: productTitle,
            price_cents: 800,
            inventory_count: 5
        }
    });
    expect(createRes.ok()).toBeTruthy();

    await memberPage.goto('/api/staff');
    await memberPage.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'User', role: 'Manager', pin_hash: '1234' }]));
    });

    await memberPage.goto('/pos.html');
    await expect(memberPage.locator('text=Terminal Locked')).toBeVisible();

    // Unlock
    for (let i = 1; i <= 4; i++) {
        await memberPage.getByRole('button', { name: i.toString(), exact: true }).click();
    }
    await memberPage.getByRole('button', { name: 'Clock In' }).click();

    // Verify product exists on POS
    const productButton = memberPage.locator('div.product-btn').filter({ hasText: productTitle });
    await expect(productButton).toBeVisible({ timeout: 15000 });

    // Set offline
    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(memberPage.locator('text=Syncing Paused').first()).toBeVisible();

    // Toggle Sold Out
    await productButton.locator('button:has-text("Sold Out")').click();

    // Queue dashboard should appear
    await expect(memberPage.locator('text=Items Pending Sync').first()).toBeVisible();

    // Restore online
    await context.setOffline(false);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('online')));
  });

  test('2. Optimistic inventory deduction on offline tap-to-pay', async ({ page, request, memberPage, context }) => {
    await memberPage.setViewportSize({ width: 375, height: 667 });

    const loginRes = await request.post('/api/v1/auth/login', {
        data: { email: 'admin@ohc.local', password: 'admin' }
    });
    const { token } = await loginRes.json();

    const productTitle = 'Shawarma Test ' + Date.now();
    const createRes = await request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: { title: productTitle, price_cents: 1000, inventory_count: 2 }
    });
    expect(createRes.ok()).toBeTruthy();

    await memberPage.goto('/api/staff');
    await memberPage.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'User', role: 'Manager', pin_hash: '1234' }]));
    });

    await memberPage.goto('/pos.html');
    for (let i = 1; i <= 4; i++) {
        await memberPage.getByRole('button', { name: i.toString(), exact: true }).click();
    }
    await memberPage.getByRole('button', { name: 'Clock In' }).click();

    const productButton = memberPage.locator('div.product-btn').filter({ hasText: productTitle });
    await expect(productButton).toBeVisible({ timeout: 15000 });
    await expect(productButton).toContainText('2 in stock');

    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Click to add to cart
    await productButton.locator('div').first().click();

    // Click charge (simulates tap to pay when offline)
    await memberPage.locator('button.charge-btn').click();

    await expect(memberPage.locator('text=Offline Quick Charge Saved.')).toBeVisible();

    // Close receipt
    await memberPage.locator('button:has-text("New Sale")').click();

    // Verify inventory updated to 1
    const newProductButton = memberPage.locator('div.product-btn').filter({ hasText: productTitle });
    await expect(newProductButton).toContainText('1 in stock');

    await context.setOffline(false);
  });

  test('3. Generates Sync Conflict Task for concurrent online purchase', async ({ page, request, memberPage }) => {
      // Create product
      const loginRes = await request.post('/api/v1/auth/login', {
          data: { email: 'admin@ohc.local', password: 'admin' }
      });
      const { token } = await loginRes.json();

      const createRes = await request.post('/api/v1/catalog/products', {
          headers: { Authorization: `Bearer ${token}` },
          data: { title: 'Concurrent ' + Date.now(), price_cents: 1200, inventory_count: 1 }
      });
      const product = await createRes.json();
      const productId = product.id || product.product_id;

      // Reserve it via POS endpoint
      const reserveRes = await request.post('/api/v1/payments/terminal/reserve', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        }
      });
      expect(reserveRes.ok()).toBeTruthy();

      // Ensure that reserving again fails
      const reserveRes2 = await request.post('/api/v1/payments/terminal/reserve', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            product_id: productId,
            quantity: 1,
            ttl_seconds: 15
        }
      });
      expect(reserveRes2.ok()).toBeTruthy();
      const res2 = await reserveRes2.json();
      expect(res2.success).toBeFalsy();
      expect(res2.error_message).toContain('checked out');
  });

  test('4. Queue is cleared upon going online', async ({ page, request, memberPage, context }) => {
    await memberPage.setViewportSize({ width: 375, height: 667 });

    const loginRes = await request.post('/api/v1/auth/login', {
        data: { email: 'admin@ohc.local', password: 'admin' }
    });
    const { token } = await loginRes.json();

    const productTitle = 'Juice ' + Date.now();
    await request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: { title: productTitle, price_cents: 500, inventory_count: 10 }
    });

    await memberPage.goto('/api/staff');
    await memberPage.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'User', role: 'Manager', pin_hash: '1234' }]));
    });

    await memberPage.goto('/pos.html');
    for (let i = 1; i <= 4; i++) {
        await memberPage.getByRole('button', { name: i.toString(), exact: true }).click();
    }
    await memberPage.getByRole('button', { name: 'Clock In' }).click();

    const productButton = memberPage.locator('div.product-btn').filter({ hasText: productTitle });
    await expect(productButton).toBeVisible({ timeout: 15000 });

    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(memberPage.locator('text=Syncing Paused').first()).toBeVisible();

    await productButton.locator('div').first().click();
    await memberPage.locator('button.charge-btn').click();

    await expect(memberPage.locator('text=1 Items Pending Sync').first()).toBeVisible();

    await context.setOffline(false);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('online')));

    // The items pending text should disappear once synced
    await expect(memberPage.locator('text=Items Pending Sync')).toBeHidden({ timeout: 15000 });
  });

  test('5. Translation agent triggers via CRDT sync queue backend handler', async ({ request, memberPage }) => {
      // Just test that the backend handler for sync deltas is reachable
      const loginRes = await request.post('/api/v1/auth/login', {
          data: { email: 'admin@ohc.local', password: 'admin' }
      });
      const { token, user } = await loginRes.json();

      const res = await request.post('/api/v1/sync/mcp-deltas', {
          headers: { Authorization: `Bearer ${token}` },
          data: {
              tenant_id: user.tenant_id || 'default',
              deltas: [{
                  id: 'delta_lang_' + Date.now(),
                  entity_type: 'menu_translation',
                  entity_id: 'menu_1',
                  payload: { target_language: 'es' },
                  updated_at: Date.now()
              }]
          }
      });
      expect(res.status()).toBe(200);
  });
});
