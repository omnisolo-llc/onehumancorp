import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { db } from './db_utils';

test.describe('KDS Offline-First KDS & Multi-Lingual Order Intake', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('KDS page handles offline data safely and allows toggle updates', async ({ page, context, loginAs, unlimitedAdminUser }) => {
    const tenant = unlimitedAdminUser.organizationId;
    const id = `kds-menu-${randomUUID()}`;
    await db.query(`INSERT INTO products (id, tenant_id, title, description, type, price, price_cents, currency, inventory_count, available_quantity, is_sold_out)
      VALUES ($1, $2, 'KDS owned menu item', 'Offline inventory fixture', 'physical', 8, 800, 'USD', 10, 10, FALSE)`, [id, tenant]);
    try {
      await loginAs(page, unlimitedAdminUser);
      await page.goto('/kitchen');
      await expect(page.getByRole('heading', { name: 'Kitchen Command Center' })).toBeVisible();
      await expect(page.getByText('Daily Menu', { exact: true })).toBeVisible();
      const toggle = page.locator(`[id="sold-out-toggle-${id}"]`);
      await expect(toggle).toHaveText('Mark Sold Out');
      await expect(page.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state', 'ready');
      await context.setOffline(true);
      await toggle.click();
      // Bind to product identity, so a label change cannot retarget the locator.
      await expect(toggle).toHaveText('Sold Out');
      await expect(page.locator('#queue-dashboard')).toContainText('1 Pending Sync');
      await context.setOffline(false);
      await expect(page.locator('#queue-dashboard')).toBeHidden({ timeout: 15000 });
      await expect(async () => {
        const rows = await db.query('SELECT is_sold_out FROM products WHERE tenant_id = $1 AND id = $2', [tenant, id]);
        expect(rows[0].is_sold_out).toBe(true);
      }).toPass({ timeout: 15000 });
      await page.reload();
      await expect(toggle).toHaveText('Sold Out');
    } finally {
      await context.setOffline(false);
      await db.query('DELETE FROM products WHERE tenant_id = $1 AND id = $2', [tenant, id]);
    }
  });
});
