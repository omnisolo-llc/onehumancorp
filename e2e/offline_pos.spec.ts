import { test, expect } from '@playwright/test';

test.describe('Offline POS Sync', () => {
  test('should enqueue and process offline POS transactions via API', async ({ request }) => {
    // 1. Post a mock batch of offline transactions
    const payload = {
      mutations: [
        {
          transaction_id: "test-offline-tx-12345",
          product_id: "test-product-id",
          quantity_deducted: 1,
          amount: 2500,
          currency: "USD",
          payment_method: "card_present"
        }
      ]
    };

    const response = await request.post('/api/v1/pos/sync', {
      data: payload,
      headers: {
        'x-spiffe-id': 'spiffe://ohc/org/test-tenant/agent/frontend'
      }
    });

    // We expect the API to return 200 OK immediately and enqueue the job.
    // Note: in a real environment, we'd setup the database via seed scripts
    // or test framework helpers and authenticate. Assuming the test harness
    // sets up `test-tenant` correctly, we expect 200.
    expect(response.status()).toBe(200);

    const body = await response.json();
    expect(body.success).toBe(true);
  });
});
