import { test, expect } from './fixtures';
import { pool } from './global-setup';

test.describe('Tap to Pay / POS Checkout API Flow', () => {
  test('POS payment intent creation and webhook deducts inventory', async ({ adminPage }) => {
    // As per the constraints: real CUJ flow without mocking the network.
    // Given the task is mostly backend ("Mobile Implementation Deferred, but API must support it"),
    // we'll verify the backend API endpoints and webhook trigger.

    const tenantId = 'e2e-tenant';
    const productId = 'prod_pos_test_1';

    // Seed product
    await pool.query(
      `INSERT INTO products (id, tenant_id, title, price_cents, inventory_count)
       VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET inventory_count = 10`,
      [productId, tenantId, 'Test POS Product', 1500, 10]
    );

    // Call create intent API using the auth from the admin page
    const intentRes = await adminPage.request.post('/api/v1/payments/terminal/intent', {
      data: {
        amount_cents: 1500,
        currency: 'usd',
        product_id: productId,
        quantity: 1,
        order_id: 'pos_order_1'
      }
    });

    // The stripe client will return 200 with the mock string in standalone/test mode.
    expect(intentRes.status()).toBe(200);
    const json = await intentRes.json();
    expect(json.Ok.client_secret).toContain('pi_mock_intent');

    // Trigger webhook manually
    const webhookRes = await adminPage.request.post('/api/v1/webhooks/stripe', {
      data: {
        type: 'payment_intent.succeeded',
        data: {
          object: {
            metadata: {
              source: 'in_person',
              tenant_id: tenantId,
              product_id: productId,
              quantity: '1',
              order_id: 'pos_order_1'
            }
          }
        }
      }
    });

    expect(webhookRes.status()).toBe(200);

    // Verify inventory deduction
    const res = await pool.query('SELECT inventory_count FROM products WHERE id = $1', [productId]);
    expect(res.rows[0].inventory_count).toBe(9);
  });
});
