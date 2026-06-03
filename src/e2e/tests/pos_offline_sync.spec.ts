import { test, expect } from '@playwright/test';

test.describe('Offline-Capable POS Sync Backend', () => {
  test('should successfully batch sync offline POS transactions', async ({ request }) => {
    // 1. Create a dummy offline transaction
    const idempotencyKey = `tx-offline-${Date.now()}`;
    const payload = {
      transactions: [
        {
          idempotency_key: idempotencyKey,
          amount_cents: 1500,
          currency: 'usd',
          payment_method: 'card_present',
          stripe_payment_intent_id: 'pi_mock_123',
          items: [
            {
              product_id: 'prod-mock-1',
              quantity: 2,
              unit_price_cents: 750
            }
          ]
        }
      ]
    };

    // 2. Mock spiffe header to simulate authenticated tenant
    const headers = {
      'x-spiffe-id': 'spiffe://ohc/org/tenant-offline-e2e/agent/x',
      'Content-Type': 'application/json'
    };

    // 3. Send the POST request to the sync endpoint
    const response = await request.post('/api/v1/pos/sync', {
      headers,
      data: payload
    });

    // 4. Verify successful enqueue
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.queued_count).toBe(1);
  });
});
