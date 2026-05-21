import { test, expect } from './fixtures';
import { Pool } from 'pg';

test.describe('Department Specific Task UIs', () => {
  let pool: Pool;

  test.beforeAll(async () => {
    pool = new Pool({
      connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
    });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test('UI: Department with no approvals shows All Caught Up directly', async ({ page }) => {
    // Ensure no pending approvals for finance
    await pool.query("UPDATE agent_approvals SET status = 'APPROVED' WHERE department = 'finance'");

    await page.goto('/team');

    await page.locator('button', { hasText: 'The Accountant' }).click();

    await expect(page.locator('h1')).toContainText('The Accountant');
    await expect(page.getByText('All Caught Up!')).toBeVisible();
    await expect(page.getByText('There are no pending actions requiring your review.')).toBeVisible();
  });

  test('UI: Rejecting a request updates the UI to All Caught Up', async ({ page }) => {
    await pool.query(`
      INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, created_at, updated_at)
      VALUES ('e2e-approval-mock-3', 'e2e-tenant', 'operations', 'Another request', 'PENDING', 'LOW', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
      ON CONFLICT (id) DO UPDATE SET status = 'PENDING';
    `);

    await page.goto('/team');

    await page.locator('button', { hasText: 'The Manager' }).click();
    await expect(page.locator('h1')).toContainText('The Manager');
    await expect(page.getByText('Another request')).toBeVisible();

    const rejectBtn = page.getByRole('button', { name: 'Reject / Edit' });
    await expect(rejectBtn).toBeVisible();
    await rejectBtn.click();

    await expect(page.getByText('All Caught Up!')).toBeVisible({ timeout: 10000 });

    const result = await pool.query('SELECT status FROM agent_approvals WHERE id = $1', ['e2e-approval-mock-3']);
    expect(result.rows[0].status).toBe('REJECTED');
  });

  test('UI: Proactive Tax & Legal Compliance Guardrails flow', async ({ page }) => {
    await pool.query(`
      INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, created_at, updated_at, feature_type)
      VALUES ('e2e-approval-mock-4', 'e2e-tenant', 'legal', 'ACTION REQUIRED: Revenue approaching EU VAT threshold. Generate and apply compliance policies?', 'PENDING', 'HIGH', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'legal_compliance')
      ON CONFLICT (id) DO UPDATE SET status = 'PENDING';
    `);

    await page.goto('/team');

    await page.locator('button', { hasText: 'The Protector' }).click();
    await expect(page.locator('h1')).toContainText('The Protector');
    await expect(page.getByText('ACTION REQUIRED: Revenue approaching EU VAT threshold. Generate and apply compliance policies?')).toBeVisible();

    // Assert the specific UI widget elements are visible
    await expect(page.getByText('Compliance Warning')).toBeVisible();
    await expect(page.getByText('Projected revenue exceeds €10,000 threshold. VAT registration and updated Privacy Policy required.')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: 'Approve' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(page.getByText('All Caught Up!')).toBeVisible({ timeout: 10000 });

    const result = await pool.query('SELECT status FROM agent_approvals WHERE id = $1', ['e2e-approval-mock-4']);
    expect(result.rows[0].status).toBe('APPROVED');
  });
});
