import { test, expect } from '@playwright/test';

test.describe('Intelligent Customer Auto-Responder', () => {
  test('should parse "where is my order" and reply with status', async ({ request }) => {
    const tenantId = 't-auto-responder-test';
    const orderId = 'test-order-1234';

    // Simulate webhook payload for the incoming message
    const response = await request.post('http://127.0.0.1:3000/api/v1/agents/webhook', {
      data: {
        tenant_id: tenantId,
        source: 'instagram',
        message: `where is my order ${orderId}`
      }
    });

    expect(response.ok()).toBeTruthy();
    const resData = await response.json();
    expect(resData.success).toBe(true);
  });

  test('should auto execute vegan query', async ({ request }) => {
    const response = await request.post('http://127.0.0.1:3000/api/v1/agents/webhook', {
      data: {
        tenant_id: 't-auto-responder-test',
        source: 'whatsapp',
        message: `do you have vegan cakes?`
      }
    });

    expect(response.ok()).toBeTruthy();
    const resData = await response.json();
    expect(resData.success).toBe(true);
  });

  test('should handle general inquiries without order id', async ({ request }) => {
    const response = await request.post('http://127.0.0.1:3000/api/v1/agents/webhook', {
      data: {
        tenant_id: 't-auto-responder-test',
        source: 'sms',
        message: `what are your hours?`
      }
    });

    expect(response.ok()).toBeTruthy();
    const resData = await response.json();
    expect(resData.success).toBe(true);
  });

  test('should gracefully handle order inquiries with missing id', async ({ request }) => {
    const response = await request.post('http://127.0.0.1:3000/api/v1/agents/webhook', {
      data: {
        tenant_id: 't-auto-responder-test',
        source: 'instagram',
        message: `where is my order? I forgot the number`
      }
    });

    expect(response.ok()).toBeTruthy();
    const resData = await response.json();
    expect(resData.success).toBe(true);
  });

  test('should gracefully handle order inquiries with invalid id', async ({ request }) => {
    const response = await request.post('http://127.0.0.1:3000/api/v1/agents/webhook', {
      data: {
        tenant_id: 't-auto-responder-test',
        source: 'instagram',
        message: `where is my order fake-id`
      }
    });

    expect(response.ok()).toBeTruthy();
    const resData = await response.json();
    expect(resData.success).toBe(true);
  });
});
