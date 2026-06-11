import { test, expect } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Omni Inbox Webhook - Identity Resolution', () => {
  test('should resolve customer by sender_id and trigger event', async ({ request, page, db }) => {
    const tenantId = 'test_tenant'; // Default testing tenant
    const customerPhone = '+1' + Math.random().toString().slice(2, 12);

    // 1. Create a customer with this phone number
    await db.query(
      `INSERT INTO customers (id, tenant_id, name, phone, email) VALUES ($1, $2, 'Test Customer', $3, 'test@example.com') ON CONFLICT DO NOTHING`,
      [uuidv4(), tenantId, customerPhone]
    );

    // 2. Send webhook payload
    const webhookPayload = {
      channel: 'whatsapp',
      sender_id: customerPhone,
      recipient_id: tenantId,
      content: 'E2E Test Message: I need help with my cake order!',
    };

    const response = await request.post('/api/v1/webhooks/inbox', {
      data: webhookPayload,
    });
    expect(response.status()).toBe(200);

    // Give it a moment to process the event loop queue
    await page.waitForTimeout(500);

    // We can also verify it in the DB
    const res = await db.query(
      `SELECT content FROM inbox_messages WHERE sender_id = $1 AND tenant_id = $2`,
      [customerPhone, tenantId]
    );
    expect(res.rows.length).toBeGreaterThan(0);
    expect(res.rows[0].content).toBe('E2E Test Message: I need help with my cake order!');
  });
});
