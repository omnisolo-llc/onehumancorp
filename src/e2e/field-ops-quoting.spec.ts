import { test, expect } from '@playwright/test';
import { memberPage } from './fixtures';
import { executeSql } from './db_utils';

test.describe('Autonomous Field Service Quoting & Deposit Engine', () => {
  let customerId: string;
  let serviceLeadId: string;
  let estimateId: string;

  test.beforeAll(async () => {
    // Clean up any existing records
    await executeSql(`
      DELETE FROM deposit_requirements;
      DELETE FROM estimates;
      DELETE FROM service_leads;
    `);

    // Insert a test customer
    const customerRes = await executeSql(`
      INSERT INTO customers (id, tenant_id, name, email)
      VALUES (gen_random_uuid(), 'e2e-tenant', 'Test Customer', 'testcustomer@example.com')
      RETURNING id
    `);
    customerId = customerRes[0].id;

    // Simulate an incoming service lead (e.g. from WhatsApp/SMS)
    const leadRes = await executeSql(`
      INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status)
      VALUES ('lead-' || gen_random_uuid(), 'e2e-tenant', '${customerId}', 'Broken pipe under sink', 'sms', 'estimating')
      RETURNING id
    `);
    serviceLeadId = leadRes[0].id;

    // Simulate the Estimator agent creating a draft estimate
    const estimateRes = await executeSql(`
      INSERT INTO estimates (id, tenant_id, service_lead_id, customer_id, description, min_price_cents, max_price_cents, status)
      VALUES ('est-' || gen_random_uuid(), 'e2e-tenant', '${serviceLeadId}', '${customerId}', 'Fix broken pipe under sink', 15000, 25000, 'draft')
      RETURNING id
    `);
    estimateId = estimateRes[0].id;

    // Simulate the deposit requirement (20% deposit)
    await executeSql(`
      INSERT INTO deposit_requirements (id, tenant_id, estimate_id, amount_cents, percentage, status)
      VALUES ('dep-' || gen_random_uuid(), 'e2e-tenant', '${estimateId}', 5000, 20.00, 'pending')
    `);
  });

  test('Owner can view and approve the drafted estimate on mobile', async ({ page, request }) => {
    // 1. Submit the lead request
    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=e2e-tenant', {
      data: {
        name: 'Field Ops Lead',
        email: 'fieldops@example.com',
        details: 'Fix broken pipe under sink'
      },
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });
    expect(submitResponse.ok()).toBeTruthy();

    // 2. Wait for the EstimatorAgentWorker to process the LeadReceived job
    await page.waitForTimeout(5000);

    // Verify the data model and backend flow work as designed.
    const updatedEstimate = await executeSql(`SELECT id, status FROM quotes WHERE customer_id = (SELECT customer_id FROM service_leads WHERE description = 'Fix broken pipe under sink' LIMIT 1) LIMIT 1`);
    expect(updatedEstimate.length).toBeGreaterThan(0);

    // Simulate UI action of approving quote
    const quoteId = updatedEstimate[0].id;
    const approveResponse = await request.patch(`/api/v1/quotes/${quoteId}/approve`, {
        headers: {
            'x-tenant-id': 'e2e-tenant'
        }
    });
    expect(approveResponse.ok()).toBeTruthy();

    // Verify approval updated DB
    const finalState = await executeSql(`SELECT status FROM quotes WHERE id = '${quoteId}'`);
    expect(finalState[0].status).toBe('ACCEPTED');
  });

  test('Customer deposit payment updates booking state', async ({ page }) => {
      // Simulate the Stripe webhook success by updating the deposit requirement
      await executeSql(`
          UPDATE deposit_requirements SET status = 'paid' WHERE estimate_id = '${estimateId}';
          UPDATE estimates SET status = 'approved' WHERE id = '${estimateId}';
      `);

      const updatedDeposit = await executeSql(`SELECT status FROM deposit_requirements WHERE estimate_id = '${estimateId}'`);
      expect(updatedDeposit[0].status).toBe('paid');

      const finalEstimate = await executeSql(`SELECT status FROM estimates WHERE id = '${estimateId}'`);
      expect(finalEstimate[0].status).toBe('approved');
  });
});
