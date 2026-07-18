import { test, expect } from '@playwright/test';

test.describe('Terminal API tests', () => {
  test('should return 401 when fetching token without auth', async ({ request }) => {
    const response = await request.get('/terminal/token');
    expect(response.status()).toBe(401);
  });

  test('should fail to create intent with missing fields', async ({ request }) => {
    const response = await request.post('/terminal/intent', {
      data: {
        currency: 'usd'
      }
    });
    expect(response.status()).toBe(400); // Or 415/422 depending on axum json extractor
  });

  test('should return 401 when capturing intent without auth', async ({ request }) => {
    const response = await request.post('/terminal/intent/capture', {
      data: {
        payment_intent_id: 'pi_123',
      }
    });
    expect(response.status()).toBe(401);
  });

  test('should fail with invalid json payload', async ({ request }) => {
    const response = await request.post('/terminal/intent', {
      headers: {
        'content-type': 'application/json'
      },
      data: 'invalid json'
    });
    expect(response.status()).toBe(400);
  });

  test('should reject invalid currency format', async ({ request }) => {
    const response = await request.post('/terminal/intent', {
      data: {
        amount_cents: 100,
        currency: ''
      }
    });
    // This will likely fail downstream or be rejected
    expect(response.ok()).toBeFalsy();
  });
});
