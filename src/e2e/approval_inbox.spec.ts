import { test, expect } from './fixtures';
import { Pool } from 'pg';

test.describe('Approval Inbox E2E', () => {
  let pool: Pool;

  test.beforeAll(async () => {
    pool = new Pool({
      connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
    });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test('User can see seeded approvals and interact with them', async ({ page }) => {
    // Re-seed the approval just in case
    await pool.query(`
      INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, created_at, updated_at)
      VALUES ('e2e-approval-mock-1', 'e2e-tenant', 'customer_success', 'Draft email for review: Maya ordered a vegan cake', 'PENDING', 'HIGH', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
      ON CONFLICT (id) DO UPDATE SET status = 'PENDING';
    `);

    await page.goto('/team');

    await page.locator('button', { hasText: 'The Ambassador' }).click();

    await expect(page.locator('h1')).toContainText('The Ambassador');
    await expect(page.getByText('Draft email for review: Maya ordered a vegan cake')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: 'Approve' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(page.getByText('All Caught Up!')).toBeVisible({ timeout: 10000 });

    const result = await pool.query('SELECT status FROM agent_approvals WHERE id = $1', ['e2e-approval-mock-1']);
    expect(result.rows[0].status).toBe('APPROVED');
  });

  test('User can see and approve another approval', async ({ page }) => {
    await pool.query(`
      INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, created_at, updated_at)
      VALUES ('e2e-approval-mock-2', 'e2e-tenant', 'marketing', 'Draft Instagram Post: New vegan cakes available!', 'PENDING', 'LOW', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
      ON CONFLICT (id) DO UPDATE SET status = 'PENDING';
    `);

    await page.goto('/team');

    await page.locator('button', { hasText: 'The Promoter' }).click();

    await expect(page.locator('h1')).toContainText('The Promoter');
    await expect(page.getByText('Draft Instagram Post: New vegan cakes available!')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: 'Approve' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(page.getByText('All Caught Up!')).toBeVisible({ timeout: 10000 });

    const result = await pool.query('SELECT status FROM agent_approvals WHERE id = $1', ['e2e-approval-mock-2']);
    expect(result.rows[0].status).toBe('APPROVED');
  });
});
