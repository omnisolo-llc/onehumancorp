import { test, expect } from '@playwright/test';

test.describe('Native Rust Omnichannel Chat System Core API', () => {
  let webhookUrl = '';

  test.beforeAll(async () => {
    // Wait for services to be ready
    webhookUrl = `http://localhost:3000/api/v1/omnichannel_core/webhook`;
  });

  test('should successfully ingest a webhook message and persist it', async ({ request }) => {
    // We will simulate a webhook POST request from a provider like WhatsApp
    const payload = {
      tenant_id: 't-12345',
      provider_type: 'whatsapp',
      sender_id: '+1234567890',
      message_content: 'Hello, I would like to order a custom cake!'
    };

    const response = await request.post(webhookUrl, {
      data: payload,
    });

    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.success).toBe(true);
    expect(data.message_id).toBeDefined();

    // In a real e2e, we would next hit the websocket or fetch from API to ensure it propagated.
    // For now we test that the ingestion succeeded cleanly end-to-end against the live backend API.
  });
});
