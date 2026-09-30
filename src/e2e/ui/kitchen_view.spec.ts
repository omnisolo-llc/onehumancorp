import { randomUUID } from 'node:crypto';
import { test, expect } from '../fixtures';
import { db } from '../db_utils';

test.describe('Kitchen View - Food Cart Daily Operations', () => {
  test('should display translated orders and support offline-first inventory toggling', async ({ page, context, loginAs, unlimitedAdminUser }) => {
    const tenantId = unlimitedAdminUser.organizationId;
    const suffix = randomUUID();
    const alice = `kitchen-e2e-${suffix}-alice`;
    const bob = `kitchen-e2e-${suffix}-bob`;
    const falafel = `kitchen-menu-${suffix}`;
    await db.query(`INSERT INTO customers (id, tenant_id, name) VALUES ($1, $3, 'Alice'), ($2, $3, 'Bob')`, [alice, bob, tenantId]);
    await db.query(`INSERT INTO products (id, tenant_id, title, description, type, price, price_cents, currency, inventory_count, available_quantity, is_sold_out)
      VALUES ($1, $2, 'Falafel Wrap', 'Kitchen test menu', 'physical', 8, 800, 'USD', 10, 10, FALSE)`, [falafel, tenantId]);
    await db.query(`INSERT INTO orders (id, tenant_id, customer_id, total_amount, status, notes, translated_notes)
      VALUES ($1, $3, $1, 8, 'pending', 'No onions, extra pita', 'بدون بصل - خبز إضافي'),
             ($2, $3, $2, 12, 'pending', 'Spicy', 'حار')`, [alice, bob, tenantId]);
    try {
      await loginAs(page, unlimitedAdminUser);
      // Wait for the real POS read/cache to expose this test's database records.
      await expect(async () => {
        const response = await page.request.get('/api/v1/pos/orders');
        expect(response.status()).toBe(200);
        const data = await response.json();
        expect(data.orders).toEqual(expect.arrayContaining([
          expect.objectContaining({ id: alice, customer_name: 'Alice', status: 'pending' }),
          expect.objectContaining({ id: bob, customer_name: 'Bob', status: 'pending' }),
        ]));
      }).toPass({ timeout: 10000 });
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/kitchen');
      await expect(page.getByRole('heading', { name: 'Kitchen Command Center' })).toBeVisible();
      const aliceCard = page.getByTestId(`kitchen-order-${alice}`);
      const ownCards = page.getByTestId(new RegExp(`^kitchen-order-kitchen-e2e-${suffix}-`));
      await expect(aliceCard).toContainText('Alice');
      await expect(aliceCard.getByText('بدون بصل')).toBeVisible();
      await expect(aliceCard.getByText('خبز إضافي')).toBeVisible();
      await expect(ownCards.getByRole('button', { name: 'Mark Ready & Notify' })).toHaveCount(2);
      const falafelToggle = page.locator(`[id="sold-out-toggle-${falafel}"]`);
      await expect(falafelToggle).toBeVisible();
      await expect(falafelToggle).toContainText('Mark Sold Out');
      await context.setOffline(true);
      await falafelToggle.click();
      await expect(falafelToggle).toContainText('Sold Out');
      await expect(page.locator('#queue-dashboard')).toBeVisible();
      await expect(page.locator('#queue-dashboard')).toContainText('1 Pending Sync');
      await context.setOffline(false);
      await aliceCard.getByRole('button', { name: 'Mark Ready & Notify' }).click();
      await expect(ownCards.getByRole('button', { name: 'Mark Ready & Notify' })).toHaveCount(1);
      await expect(page.getByTestId(`kitchen-order-${bob}`)).toBeVisible();
      await expect(page.locator('#queue-dashboard')).toBeHidden({ timeout: 15000 });
      await expect(async () => {
        const order = await db.query('SELECT status FROM orders WHERE tenant_id = $1 AND id = $2', [tenantId, alice]);
        const product = await db.query('SELECT is_sold_out FROM products WHERE tenant_id = $1 AND id = $2', [tenantId, falafel]);
        expect(order[0].status).toBe('ready');
        expect(product[0].is_sold_out).toBe(true);
      }).toPass({ timeout: 15000 });
    } finally {
      await context.setOffline(false);
      await db.query('DELETE FROM orders WHERE tenant_id = $1 AND id IN ($2, $3)', [tenantId, alice, bob]);
      await db.query('DELETE FROM customers WHERE tenant_id = $1 AND id IN ($2, $3)', [tenantId, alice, bob]);
      await db.query('DELETE FROM products WHERE tenant_id = $1 AND id = $2', [tenantId, falafel]);
    }
  });
});
