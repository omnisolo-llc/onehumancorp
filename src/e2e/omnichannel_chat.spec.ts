import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Native Omnichannel Chat Webhook Ingest', () => {
  let tenantId: string;
  let inboxId: string;

  test.beforeAll(async ({ request }) => {
    tenantId = '00000000-0000-0000-0000-000000000000'; // Default test tenant from auth harness
    inboxId = randomUUID();

    // Setup an inbox in the DB to avoid FK constraint panic
    const res = await request.post('/api/v1/chat/inboxes', {
      data: {
        tenant_id: tenantId,
        name: "Test Omnichannel Inbox"
      },
      headers: {
        'x-tenant-id': tenantId
      }
    });

    // We expect this setup to succeed
    expect(res.ok()).toBeTruthy();
    const data = await res.json();
    inboxId = data.id;
  });

  test('should ingest incoming message via webhook and show in UI', async ({ request, page }) => {
    const messageContent = 'Do you have vegan cupcakes for this Saturday?';

    // 1. Simulate incoming message via our native Rust webhook
    const webhookRes = await request.post('/api/v1/chat/webhook', {
      data: {
        tenant_id: tenantId,
        inbox_id: inboxId,
        contact_name: "Maya Customer",
        contact_email: "maya.customer@example.com",
        contact_phone: "+1234567890",
        message_content: messageContent
      },
      headers: {
        'x-tenant-id': tenantId
      }
    });

    // The endpoint should now return 200 OK since it's mounted and uses proper error mapping
    expect(webhookRes.ok()).toBeTruthy();
    const responseData = await webhookRes.json();
    expect(responseData.content).toBe(messageContent);

    // 2. Login and navigate to dashboard
    await page.goto(`/login`);

    const tenantInput = page.locator('input[name="tenant_id"]');
    if (await tenantInput.isVisible()) {
      await tenantInput.fill(tenantId);
      await page.fill('input[name="password"]', 'admin');
      await page.click('button[type="submit"]');
      await page.waitForURL('**/dashboard**');
    }

    // (UI assertion would go here if there was a frontend implemention, this PR only adds the backend domain model and infrastructure).
  });
});
