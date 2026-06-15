import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

test.describe('Food Cart Autonomous Offline Operations Flow', () => {
  let pool: Pool;
  const tenantId = 'e2e-tenant-food-cart';
  const item1Id = 'inv-e2e-1';

  test.beforeAll(async () => {
    // Only use database if it's available locally
    try {
      pool = new Pool({
        connectionString: process.env.OHC_DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/ohc',
      });
      await pool.query('SELECT 1');

      // Seed data
      await pool.query(`
        INSERT INTO tenants (id, name, created_at, updated_at)
        VALUES ($1, 'Fatima Food Cart', NOW(), NOW())
        ON CONFLICT (id) DO NOTHING
      `, [tenantId]);

      await pool.query(`
        INSERT INTO products (id, tenant_id, title, price_cents, inventory_count, is_sold_out)
        VALUES ($1, $2, 'Falafel Wrap', 800, 10, false)
        ON CONFLICT (id) DO UPDATE SET is_sold_out = false
      `, [item1Id, tenantId]);

      await pool.query(`
        INSERT INTO orders (id, tenant_id, customer_name, total_amount, status)
        VALUES ('order-1', $1, 'Ahmed', 800, 'Received')
        ON CONFLICT (id) DO UPDATE SET status = 'Received'
      `, [tenantId]);
    } catch (e) {
      console.log('Database not available, skipping seeding');
      pool = null as any;
    }
  });

  test.afterAll(async () => {
    if (pool) {
      await pool.end();
    }
  });

  test('Fatima can view orders, toggle sold-out offline, and sync when online', async ({ page, context }) => {
    // Navigate to KDS
    await page.addInitScript((tenantId) => {
      window.localStorage.setItem('tenant_id', tenantId);
    }, tenantId);

    // We expect the KDS page to load, but we skip assertions since there's no backend running in CI
    test.skip(!pool, 'Database not running');

    await page.goto('/pos/kds');
    await expect(page.locator('h1')).toContainText('Kitchen Display System');

    // Arabic translation check
    await page.getByTestId('lang-toggle').click();
    await expect(page.locator('h1')).toContainText('نظام عرض المطبخ');

    // Simulate going offline
    await context.setOffline(true);

    // Toggle Sold Out offline
    const toggleBtn = page.getByTestId(`toggle-soldout-${item1Id}`);
    await expect(toggleBtn).toBeVisible();
    await toggleBtn.click();

    // Verify UI updates optimistically
    await expect(toggleBtn).toHaveClass(/bg-red-500/);

    // Simulate clicking order ready
    const prepareBtn = page.getByTestId('btn-prepare-order-1');
    await expect(prepareBtn).toBeVisible();
    await prepareBtn.click();

    const readyBtn = page.getByTestId('btn-ready-order-1');
    await expect(readyBtn).toBeVisible();
    await readyBtn.click();

    // Go back online and wait for sync
    await context.setOffline(false);

    // Give sync manager time to flush queue
    await page.waitForTimeout(2000);

    // Verify DB sync
    if (pool) {
        const res = await pool.query('SELECT is_sold_out FROM products WHERE id = $1', [item1Id]);
        expect(res.rows[0].is_sold_out).toBe(true);

        const orderRes = await pool.query('SELECT status FROM orders WHERE id = $1', ['order-1']);
        expect(orderRes.rows[0].status).toBe('Ready');
    }
  });
});
