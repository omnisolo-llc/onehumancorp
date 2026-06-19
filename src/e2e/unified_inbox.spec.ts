import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Unified Inbox Webhook & Triage Feed', () => {
  test('should ingest a message via webhook and expose it on triage feed', async ({ request }) => {
    const tenantId = 'e2e-tenant';
    const senderId = `user_${randomUUID()}@example.com`;
    const messageContent = 'Can you fix my sink tomorrow?';

    // 1. Post to webhook
    const postResponse = await request.post(`/api/unified-inbox/webhook`, {
      data: {
        tenant_id: tenantId,
        channel_type: 'instagram',
        sender_id: senderId,
        content: messageContent,
      }
    });

    expect(postResponse.ok()).toBeTruthy();
    const postBody = await postResponse.json();
    expect(postBody.success).toBe(true);
    expect(postBody.message_id).toBeDefined();

    const messageId = postBody.message_id;

    // 2. Fetch triage feed
    const getResponse = await request.get(`/api/unified-inbox/triage-feed?tenant_id=${tenantId}`);
    expect(getResponse.ok()).toBeTruthy();

    const getBody = await getResponse.json();
    expect(getBody.messages).toBeDefined();

    // 3. Verify message is in feed
    const found = getBody.messages.find((m: any) => m.id === messageId);
    expect(found).toBeDefined();
    expect(found.channel_type).toBe('instagram');
    expect(found.content).toBe(messageContent);
    expect(found.sender_id).toBe(senderId);
  });
});
