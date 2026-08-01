import { expect, test } from '@playwright/test';

test.describe('Native Omnichannel Chat Webhook and WS', () => {
  test('should process a webhook payload and verify it in the backend', async ({ request }) => {
    const tenantId = 'tenant_1';
    const inboxId = 'inbox_1';
    const payload = {
        tenant_id: tenantId,
        inbox_id: inboxId,
        channel_type: 'whatsapp',
        sender_id: '+19999999999',
        sender_name: 'Test Customer',
        message: 'Hello from E2E Test'
    };

    // Send the webhook request
    const response = await request.post('/api/v1/native_chat/webhook', {
        data: payload,
    });

    // It should be successful
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.message_id).toBeDefined();
  });
});
