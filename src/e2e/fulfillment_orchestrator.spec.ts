import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

test.describe('Fulfillment Orchestrator Master Proposal', () => {
  let pool: Pool;

  test.beforeAll(async () => {
    const dbUrl = process.env.DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/ohc';
    pool = new Pool({ connectionString: dbUrl });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test('Maya can approve a coordinated fulfillment draft from a new inquiry', async ({ page }) => {

    // We simulate Maya logging in
    await page.goto('/login');
    await page.fill('input[name="email"]', 'e2e@example.com');
    await page.fill('input[name="password"]', 'password');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/dashboard', { timeout: 15000 });


    // Clean up any previous runs
    await pool.query(`DELETE FROM agent_feed_items WHERE id = 'e2e-fulfillment-test-1'`);

    // Insert `triage.inquiry` job
    const inquiryJobId = 'job-e2e-inquiry-' + Date.now();
    const inquiryPayload = JSON.stringify({
      message_id: 'msg-e2e-fulfillment',
      customer_id: 'cust-sarah-123',
      sender_id: 'sarah_insta',
      source: 'instagram',
      content: 'I want a vegan cake Friday',
      action_type: 'Draft Quote',
      action_payload: JSON.stringify({ total_amount_cents: 5500, required_deposit_cents: 1000 }),
      context_summary: 'Custom Cake',
      priority: 'High',
      event_source: 'instagram_dm'
    });

    await pool.query(`
      INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
      VALUES ($1, process.env.TEST_TENANT_ID || 'tenant_123', 'triage.inquiry', $2, 'PENDING', NOW())
    `, [inquiryJobId, inquiryPayload]);

    // Go to the unified agent feed
    await page.goto('/dashboard');

    // The Fulfillment Orchestrator Worker should pick up triage.inquiry and output a fulfillment_draft
    const fulfillmentCard = page.locator('text=Fulfillment Draft: Custom Cake');
    await expect(fulfillmentCard).toBeVisible({ timeout: 15000 });

    // Verify proof checks are present
    await expect(page.locator('text=✅ Surge pricing applied (+15%).')).toBeVisible();
    await expect(page.locator('text=Your total is $55.00 and we require a deposit of $10.00.')).toBeVisible();

    // Click Approve & Send
    const approveBtn = page.getByTestId('feed-approve-btn').filter({ hasText: 'Approve & Send' }).first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // We should expect the action to succeed
    await expect(fulfillmentCard).toBeHidden({ timeout: 10000 });
  });
});
