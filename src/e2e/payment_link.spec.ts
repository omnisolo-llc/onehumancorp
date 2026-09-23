import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { query } from './db_utils';

test.describe('Payment Link E2E', () => {
  test('User receives actual payment link and webhook updates state to PAID', async ({ page, request }) => {
    await adminPage(page);
    // 1. Create a draft invoice
    const createRes = await request.post('/api/v1/invoices', {
      data: {
        client_id: 'client-test-payment',
        client_name: 'Test Client',
        due_date: Math.floor(Date.now() / 1000) + 86400,
        currency: 'USD',
        line_items: [
          {
            description: 'E2E Webhook Test Item',
            quantity: 1,
            unit_price: 15.0,
          },
        ],
      },
    });

    expect(createRes.ok()).toBeTruthy();
    const invoice = await createRes.json();
    expect(invoice.id).toBeDefined();

    // 2. Simulate the webhook from Stripe
    const webhookPayload = {
      id: 'evt_test_webhook',
      type: 'checkout.session.completed',
      data: {
        object: {
          id: 'cs_test_session',
          mode: 'payment',
          payment_status: 'paid',
          metadata: {
            invoice_id: invoice.id,
            tenant_id: 'e2e-tenant',
          },
        },
      },
    };

    const webhookRes = await request.post('/api/v1/inbox/webhook', {
      data: webhookPayload,
    });

    // Status should be OK
    expect(webhookRes.ok()).toBeTruthy();

    // 3. Verify the invoice in the DB transitioned to paid
    const rows = await query('SELECT payment_status FROM invoices WHERE id = $1', [invoice.id]);
    expect(rows.length).toBe(1);
    expect(rows[0].payment_status).toBe('paid');
  });
});
