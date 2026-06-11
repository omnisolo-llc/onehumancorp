import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { Pool } from 'pg';

test.describe('Omnichannel Returns & Exchange Orchestrator', () => {
  test('Owner can approve a return request from the feed', async ({ browser }) => {
    // We use adminPage fixture which signs in as an e2e user.
    // Instead of using adminPage directly via fixture injection (since we must use page),
    // we'll follow the pattern from other tests if needed, or just use the page.

    // We'll create a new page and sign in as the E2E user.
    const context = await browser.newContext();
    const page = await context.newPage();

    // Setup - login
    await page.goto('/login');
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('**/dashboard');

    // 1. Owner opens the returns page
    await page.goto('/returns');

    // 2. Owner feed shows actionable card
    const card = page.locator('[data-testid="return-card-return_e2e_123"]');
    await expect(card).toBeVisible();
    await expect(card).toContainText('Order #ORD-4001');
    await expect(card).toContainText('$45.00');

    // 3. Click Approve
    const approveBtn = page.locator('[data-testid="approve-btn-return_e2e_123"]');
    await approveBtn.click();

    // 4. Verify processing state and success message
    await expect(page.locator('text=Processing...')).toBeVisible();
    await expect(page.locator('text=Return Approved Successfully')).toBeVisible({ timeout: 10000 });

    // 5. Verify the button is gone (status changed)
    await expect(approveBtn).not.toBeVisible();
    await expect(card).toContainText('PROCESSED');

    // 6. Verify database backend state directly via pg module
    const connectionString = process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc';
    const pool = new Pool({ connectionString });

    // Verify inventory restock in the ledger
    const ledgerRes = await pool.query(`
      SELECT * FROM ohc_universal_ledger
      WHERE tenant_id = 'e2e-tenant' AND action_type = 'INVENTORY_RESTOCK' AND state_change->>'product_id' = 'prod_e2e_001'
      ORDER BY created_at DESC LIMIT 1
    `);
    expect(ledgerRes.rows.length).toBeGreaterThan(0);
    const ledgerEntry = ledgerRes.rows[0];
    expect(ledgerEntry.state_change.quantity_added).toBe(1);

    // Verify Stripe refund status
    const returnRes = await pool.query(`
      SELECT stripe_refund_id, status FROM return_requests WHERE id = 'return_e2e_123'
    `);
    expect(returnRes.rows[0].status).toBe('processed');
    expect(returnRes.rows[0].stripe_refund_id).toBeTruthy(); // should be the mock prefix 're_test_pi_mock_123'

    await pool.end();
  });
});
