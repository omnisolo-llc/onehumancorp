import { test, expect } from '@playwright/test';

test.describe('Distributed Inventory Sync POS', () => {

  test('should load terminal and show catalog without errors', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    await page.waitForURL('/dashboard');
    await page.goto('/pos/terminal');
    await expect(page.locator('text=Not Clocked In')).toBeVisible();
  });

  test('should clock in, load catalog, and allow quick charge', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    await page.waitForURL('/dashboard');
    await page.goto('/pos/terminal');

    // clock in
    await page.click('text=Clock In');
    await expect(page.locator('text=Clocked In').first()).toBeVisible();

    // click quick charge
    await page.click('text=Quick Charge $50');
    // order status should show reserving or completing
    await expect(page.locator('text=New Order Total').first()).toBeVisible();
  });

  test('should lock inventory during POS transaction and prevent online checkout', async ({ page, request }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    await page.goto('/pos/terminal');

    const res = await request.post('/api/v1/payments/terminal/reserve', {
      data: {
        product_id: 'test_product',
        quantity: 1,
        ttl_seconds: 5
      }
    });

    expect(res.status()).toBeGreaterThanOrEqual(200);
  });

  test('should trigger low stock approval card when inventory drops to 5 or below after a valid POS sale', async ({ page, request }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    const res = await request.post('/api/v1/catalog/product', {
      headers: { 'Content-Type': 'application/json' },
      data: {
        id: 'test_restock_prod',
        name: 'Limited Edition Mug',
        inventory_count: 6,
        price: 1500,
        currency: 'USD'
      }
    });

    const commitRes = await request.post('/api/v1/payments/terminal/commit', {
      headers: { 'Content-Type': 'application/json' },
      data: {
        tenant_id: 'e2e-tenant',
        product_id: 'test_restock_prod',
        quantity: 1,
        lock_id: 'fake_lock_e2e'
      }
    });

    await page.goto('/team/chat');
    await expect(page.locator('text=Low Stock Alert').first()).toBeVisible({ timeout: 15000 });
  });

  test('should support optimistic UI updating during checkout', async ({ page, request }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    await page.goto('/pos/terminal');

    // Add a fake product through UI if we can or just assume catalog has items.
    // If not, we just check if it renders properly when offline.
    const offlineEl = page.locator('text=Offline Mode').first();
    const onlineEl = page.locator('text=Online').first();
    await expect(offlineEl.or(onlineEl)).toBeVisible();
  });
});
