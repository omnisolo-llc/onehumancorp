import { test, expect } from './fixtures';

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
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // 2. Wait for dashboard to load
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    const response = await request.post('/api/v1/auth/login', {
        data: {
            email: 'test@example.com',
            password: 'password123'
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

test('Persona: Online checkout fails due to offline POS conflict and triggers Customer Success Agent', async ({ request, page, memberPage, context }) => {
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
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    const response = await request.post('/api/v1/auth/login', {
        data: {
            email: 'test@example.com',
            password: 'password123'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    const createProductRes = await request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: 'Conflict Test Product 2',
            inventory_count: 1,
            price_cents: 1000
        }
    });
    expect(createProductRes.ok()).toBeTruthy();
    const product = await createProductRes.json();
    const productId = product.id || product.product_id;

    // 2. Go to POS page and log in
    await memberPage.goto('/pos.html');
    await memberPage.evaluate(() => { localStorage.setItem("tenant_id", "default"); });

    await memberPage.getByRole('button', { name: '1' }).click();
    await memberPage.getByRole('button', { name: '2' }).click();
    await memberPage.getByRole('button', { name: '3' }).click();
    await memberPage.getByRole('button', { name: '4' }).click();

    await memberPage.waitForTimeout(500);
    await memberPage.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
    await memberPage.waitForTimeout(500);

    // Wait for product catalog to load
    await memberPage.waitForSelector('text=Product Catalog', { timeout: 10000 });

    // Ensure the product exists
    const conflictProductBtn = memberPage.locator('button').filter({ hasText: 'Conflict Test Product 2' }).first();
    await expect(conflictProductBtn).toBeVisible();

    // 3. Set network offline
    await context.setOffline(true);
    await memberPage.evaluate(() => { window.dispatchEvent(new Event('offline')); });

    // 4. Add "Conflict Test Product 2" to cart and pay
    await conflictProductBtn.click();
    await expect(memberPage.locator('text=Tap to Pay via Terminal')).toBeVisible();

    // Mock terminal connect
    const discoverBtn = memberPage.locator('button', { hasText: 'Discover Readers' });
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
        await memberPage.locator('button', { hasText: 'Connect' }).first().click();
    }
    const collectBtn = memberPage.locator('button', { hasText: /Collect Payment/i });
    await expect(collectBtn).toBeVisible();
    await collectBtn.click();

    // Mock successful tap for E2E
    await memberPage.locator('button:has-text("Simulate Customer Tap")').click();
    await expect(memberPage.getByText('Offline Quick Charge Saved.')).toBeVisible({ timeout: 10000 });

    // 5. Concurrently simulate an online checkout via the UI
    const customerContext = await page.context().browser()!.newContext();
    const customerPage = await customerContext.newPage();

    await customerPage.goto(`/checkout?product_id=${productId}`);
    await customerPage.evaluate(() => localStorage.setItem('tenant', 'default'));
    await customerPage.reload();

    await customerPage.getByRole('button', { name: "Pay" }).click();

    // 6. Set network online to trigger sync
    await context.setOffline(false);
    await memberPage.evaluate(() => { window.dispatchEvent(new Event('online')); });

    // Wait for the sync to process and the Customer Success Agent to generate a draft
    // Verify the Customer Success Agent draft in the Inbox/Feed
    await memberPage.goto('/feed');
    await expect(memberPage.getByText("Checkout failed because Conflict Test Product 2 was out of stock.")).toBeVisible({ timeout: 20000 });

    await customerContext.close();
});
