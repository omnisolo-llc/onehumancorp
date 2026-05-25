import { test, expect } from './fixtures';
import { Pool } from 'pg';

test.describe('Success Milestones', () => {
  let pool: Pool;

  test.beforeAll(async () => {
    pool = new Pool({
      connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
    });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test('should show 10th order milestone popup only when 10 orders are reached', async ({ page, request }) => {
    // Generate a unique tenant ID for this test
    const tenantId = `test-tenant-${Date.now()}`;

    // Seed tenant and user
    await pool.query(`
      INSERT INTO tenants (id, name, industry, tier)
      VALUES ($1, 'Milestone Test Bakery', 'Food', 'starter')
    `, [tenantId]);

    await pool.query(`
      INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id)
      VALUES ($1, 'milestone_user', 'test@example.com', '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a', ARRAY['ADMIN'], TRUE, $2)
    `, [`user-${tenantId}`, tenantId]);

    // Go to login and authenticate
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.fill('input[placeholder="Organization ID"]', tenantId);
    await page.click('button:has-text("Sign in")');

    await expect(page).toHaveURL(/.*dashboard.*/);

    // With 0 orders, milestone should not appear
    await expect(page.getByText('10th Order!', { exact: false })).not.toBeVisible();

    // Insert 10 orders
    for (let i = 1; i <= 10; i++) {
        await pool.query(`
          INSERT INTO orders (id, tenant_id, customer_id, total_amount, status)
          VALUES ($1, $2, 'test-customer', 10.00, 'ready')
        `, [`order-${tenantId}-${i}`, tenantId]);
    }

    // Clear local storage flag to allow it to show again just in case, though it shouldn't have been set
    await page.evaluate(() => {
        localStorage.removeItem('10th_order_milestone_shown');
    });

    // Reload page
    await page.reload();
    await page.waitForTimeout(2000); // Give it time to fetch

    // The milestone should now appear
    await expect(page.getByText('10th Order!', { exact: false })).toBeVisible({ timeout: 10000 });

    // Check that there's a button to dismiss it
    const closeBtn = page.locator('.bg-white.w-full.max-w-md button').first();
    await closeBtn.click();

    await expect(page.getByText('10th Order!', { exact: false })).not.toBeVisible();
  });
});
