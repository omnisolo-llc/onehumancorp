import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Omnichannel Inbox E2E', () => {
  const tenantId = 'omni-test-tenant-' + uuidv4().slice(0, 8);

  test('should receive message, create draft with context, and allow approval', async ({ page, request }) => {
    // 1. Setup: Create a customer with some history
    // We'll use the internal API or direct DB if needed, but here we assume the system can resolve
    // Let's simulate a webhook for a new message
    const senderId = 'cakefan_ig';
    const messageContent = 'I want to order a vegan cake for next Friday.';

    // Mocking customer history via DB seeding would be better, but let's see if we can trigger the flow
    const webhookResponse = await request.post('/api/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram',
        sender_id: senderId,
        message: messageContent
      }
    });

    expect(webhookResponse.ok()).toBeTruthy();
    const { message_id } = await webhookResponse.json();
    expect(message_id).toBeDefined();

    // 2. Wait for Agent to process and create a draft
    // In a real environment, this is async. We might need to poll or wait.
    // For the test, we'll wait a bit.
    await new Promise(resolve => setTimeout(resolve, 3000));

    // 3. Login and check the Mobile Feed (375px)
    await page.setViewportSize({ width: 375, height: 812 });

    // Perform login (assuming a helper or direct cookie set if possible, otherwise UI login)
    // For OHC, we usually have a dev login or auto-login for tests
    await page.goto('/login');
    await page.fill('input[name="username"]', 'admin'); // Assuming default test creds
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');

    await page.goto('/feed');

    // 4. Verify the Action Card appears
    const card = page.locator('[data-testid="agent-feed-card"]').first();
    await expect(card).toBeVisible();
    await expect(card).toContainText('Draft Reply: instagram');
    await expect(card).toContainText(messageContent);

    // 5. Approve the draft
    await card.locator('[data-testid="feed-approve-btn"]').click();

    // 6. Verify card is removed (optimistic UI)
    await expect(card).not.toBeVisible();

    // 7. Verify DB state (optional via API)
    const inboxRes = await request.get(`/api/ui/inbox/messages?tenant_id=${tenantId}`);
    const messages = await inboxRes.json();
    const updatedMessage = messages.find((m: any) => m.id === message_id);
    // Note: status might be 'auto_replied' or 'sent' depending on agent config
    expect(['auto_replied', 'sent', 'replied']).toContain(updatedMessage.status);
  });
});
