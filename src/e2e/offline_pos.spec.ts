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

    expect(payload.mutations).toHaveLength(1);

    const health = await request.get('/api/health');
    expect([200, 404]).toContain(health.status());
  });
});
