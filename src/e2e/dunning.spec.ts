import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

test.describe('Intelligent Accounts Receivable & Dunning Engine E2E', () => {
  let pool: Pool;

  test.beforeAll(async () => {
    // Setup a DB connection for setting up real E2E data
    const dbUrl = process.env.DATABASE_URL || 'postgres://ohc:ohc_local_pass@localhost:5432/ohc';
    pool = new Pool({ connectionString: dbUrl });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test('Owner approves invoice reminder via Action Card', async ({ page, request }) => {
    // 1. Setup Data
    const tenantId = 'e2e-tenant';
    const customerId = 'cust_dunning_123';
    const invoiceId = 'inv_dunning_456';
    const eventId = 'feed_dunning_789';

    // Ensure customer exists
    await pool.query(`
      INSERT INTO customers (id, tenant_id, name, email)
      VALUES ($1, $2, $3, $4)
      ON CONFLICT (id) DO NOTHING
    `, [customerId, tenantId, 'Acme Corp', 'billing@acme.com']);

    // Ensure invoice exists and is overdue
    await pool.query(`
      INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, total_amount_cents, payment_status, view_count, amount_paid_cents)
      VALUES ($1, $2, $3, $4, 'sent', extract(epoch from now() - interval '3 days'), 'USD', 1200.0, 120000, 'unpaid', 0, 0)
      ON CONFLICT (id) DO NOTHING
    `, [invoiceId, tenantId, customerId, 'Acme Corp']);

    // Insert the agent feed item mimicking the finance agent
    await pool.query(`
      INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
      VALUES ($1, $2, 'finance_agent', $3, $4, 'PENDING_APPROVAL', NOW(), NOW())
      ON CONFLICT (id) DO UPDATE SET lifecycle_state = 'PENDING_APPROVAL'
    `, [
      eventId,
      tenantId,
      JSON.stringify({
         feature_type: 'invoice_followup',
         original_message: 'Invoice inv_dunning_456 for $1200 is 3 days past due.',
         generated_response: 'Hi Acme Corp, just a gentle nudge that invoice inv_dunning_456 for $1,200 is slightly overdue. Let me know if you need anything!'
      }),
      JSON.stringify({
        action_type: 'Send Reminder',
        feature_type: 'invoice_followup',
        invoice_id: invoiceId,
        customer_id: customerId,
        original_message: 'Invoice inv_dunning_456 for $1200 is 3 days past due.',
        generated_response: 'Hi Acme Corp, just a gentle nudge that invoice inv_dunning_456 for $1,200 is slightly overdue. Let me know if you need anything!'
      })
    ]);

    // 2. Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible();

    // 3. Verify UI reflects the database state
    await page.reload();

    await expect(page.getByText('Action Required: Overdue Invoice')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Invoice inv_dunning_456 for $1200 is 3 days past due.')).toBeVisible();
    await expect(page.getByText('Hi Acme Corp, just a gentle nudge')).toBeVisible();

    // 4. Act (Approve & Send via real API)
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // The feed should clear because of optimistic update and the state change
    await expect(page.getByText('Action Required: Overdue Invoice')).not.toBeVisible();

    // 5. Verify Backend State (Ensure the real API updated the DB via handle_invoice_action)
    // Give it a brief moment for the background domain action to execute
    await page.waitForTimeout(2000);

    const commEventsResult = await pool.query(`SELECT status, channel, drafted_content FROM invoice_communication_events WHERE invoice_id = $1`, [invoiceId]);
    expect(commEventsResult.rows.length).toBeGreaterThan(0);
    expect(commEventsResult.rows[0].status).toBe('sent');
    expect(commEventsResult.rows[0].drafted_content).toContain('Hi Acme Corp, just a gentle nudge');
  });
});
