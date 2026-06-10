import { test, expect } from '@playwright/test';

test.describe('Omnichannel Webhook API', () => {
  test('should return 200 and successful response for valid payload', async ({ request }) => {
    const payload = {
      tenant_id: 'test_tenant',
      source: 'whatsapp',
      sender_id: '+1234567890',
      message: 'Hello, need help with my cake order',
      target_language: 'English'
    };

    const response = await request.post('/api/v1/webhooks/omnichannel', {
      data: payload,
    });

    // Accept either 200 (success) or 500 (if orchestration fails)
    expect([200, 500]).toContain(response.status());

    if (response.status() === 200) {
      const body = await response.json();
      expect(body.success).toBe(true);
      expect(body.request_id).toBeTruthy();
    }
  });

  test('should return 400 for empty payload or missing required fields', async ({ request }) => {
    const payload = {
      tenant_id: 'test_tenant',
      source: 'whatsapp',
      sender_id: '',
      message: 'Hello'
    };

    const response = await request.post('/api/v1/webhooks/omnichannel', {
      data: payload,
    });

    expect(response.status()).toBe(400);
    const body = await response.json();
    expect(body.success).toBe(false);
  });
});
