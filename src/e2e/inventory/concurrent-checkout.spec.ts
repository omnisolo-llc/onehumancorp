import { test, expect } from '@playwright/test';
import { E2E_ADMIN_USER } from '../identities';
import { db } from '../db_utils';

test.describe('Omnichannel Tap-to-Pay and Inventory Sync Engine Architecture', () => {
  test.beforeAll(async () => {
    // Setup initial product for test
    await db.query(`
      INSERT INTO products (id, tenant_id, name, inventory_count, available_quantity)
      VALUES ('test-concurrent-prod', $1, 'Concurrent Checkout Item', 1, 1)
      ON CONFLICT (id, tenant_id) DO UPDATE SET inventory_count = 1, available_quantity = 1, locked_quantity = 0
    `, [E2E_ADMIN_USER.organizationId]);
  });

  test.afterAll(async () => {
    await db.query(`DELETE FROM products WHERE id = 'test-concurrent-prod'`);
  });

  test('E2E Playwright tests simulating concurrent checkouts', async ({ request, baseURL }) => {
    const orgId = E2E_ADMIN_USER.organizationId;
    const checkoutPayload = {
      tenant_id: orgId,
      type: "IN_PERSON",
      amount_cents: 5000,
      cart_payload: { items: [{
        product: { id: "test-concurrent-prod" },
        quantity: 1
      }]}
    };

    // Simulate two simultaneous checkout requests
    const p1 = request.post(`${baseURL}/api/v1/checkout/session`, { data: checkoutPayload });
    const p2 = request.post(`${baseURL}/api/v1/checkout/session`, { data: checkoutPayload });

    const [res1, res2] = await Promise.all([p1, p2]);
    const body1 = await res1.json();
    const body2 = await res2.json();

    // One should succeed, one should fail
    const successes = [body1.success, body2.success].filter(Boolean).length;
    const errors = [body1.error_message, body2.error_message].filter(Boolean);

    expect(successes).toBe(1);
    expect(errors).toContain("Item is currently being checked out by another customer.");
  });
});
