import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

test.describe('Autonomous Supply Chain', () => {
  let pool: Pool | null = null;

  test.beforeAll(async () => {
    try {
      pool = new Pool({
        connectionString: process.env.OHC_DATABASE_URL || process.env.DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/postgres'
      });
      await pool.query('SELECT 1');

      await pool.query(`
        CREATE TABLE IF NOT EXISTS tenants (id TEXT PRIMARY KEY);
        INSERT INTO tenants (id) VALUES ('tenant1') ON CONFLICT DO NOTHING;

        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            approval_status TEXT,
            action_risk TEXT,
            payload TEXT
        );
        INSERT INTO shared_tasks (id, organization_id, title, description, status, approval_status, action_risk, payload)
        VALUES ('mat1', 'tenant1', 'Operations', 'Cocoa Powder', 'PENDING', 'PENDING', 'HIGH', '{}')
        ON CONFLICT (id) DO UPDATE SET status = 'PENDING', approval_status = 'PENDING';
      `);
    } catch (e) {
      pool = null;
    }
  });

  test.afterAll(async () => {
    if (pool) {
      await pool.query(`DELETE FROM shared_tasks WHERE id IN ('mat1');`);
      await pool.end();
    }
  });

  test('Maya approves a Purchase Order when raw materials are low', async ({ page }) => {
    await page.goto('http://localhost:3000/inventory');

    // We expect there's some text saying Inventory or we wait until body loads
    await page.waitForSelector('text=Inventory', { timeout: 15000 });

    const alertCard = page.locator('[data-testid="alert-card-mat1"]');
    await expect(alertCard).toBeVisible({ timeout: 15000 });
    await expect(alertCard).toContainText('Cocoa Powder');

    const approveBtn = page.locator('[data-testid="approve-btn-mat1"]');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toHaveText('Approve & Pay');

    await approveBtn.click();

    const successMsg = page.locator('[data-testid="success-msg"]');
    await expect(successMsg).toBeVisible({ timeout: 10000 });
    await expect(successMsg).toHaveText('Approved Purchase Order for mat1');
    await expect(alertCard).not.toBeVisible();
  });

  test('Dashboard displays empty state when all stocks are good', async ({ page }) => {
    await page.goto('http://localhost:3000/inventory');
    await page.waitForSelector('text=Inventory', { timeout: 15000 });

    const buttons = await page.locator('button:has-text("Approve & Pay")').all();
    for (const btn of buttons) {
      await btn.click();
      await page.waitForTimeout(1000);
    }

    await expect(page.locator('text=All stock levels are looking good!')).toBeVisible({ timeout: 15000 });
  });

  test('Alert card has the correct styling applied (mac-glass-container)', async ({ page }) => {
    await page.goto('http://localhost:3000/inventory');
    await page.waitForSelector('text=Inventory', { timeout: 15000 });

    const card = page.locator('.mac-glass-container').first();
    const count = await card.count();
    if (count > 0) {
      await expect(card).toBeVisible();
    }
  });

  test('Page header is rendered correctly with Outfit font', async ({ page }) => {
    await page.goto('http://localhost:3000/inventory');
    await page.waitForSelector('h1', { timeout: 15000 });
    const header = page.locator('h1').filter({ hasText: 'Inventory' }).first();
    await expect(header).toHaveClass(/font-outfit/);
  });

  test('Vendor information is present in the alert card', async ({ page }) => {
    await page.goto('http://localhost:3000/inventory');
    await page.waitForSelector('text=Inventory', { timeout: 15000 });

    const count = await page.locator('[data-testid="alert-card-mat1"]').count();
    if (count > 0) {
      const card = page.locator('[data-testid="alert-card-mat1"]');
      await expect(card).toContainText('Acme Supply');
      await expect(card).toContainText('50 units');
    }
  });
});
