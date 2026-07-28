import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat', () => {
  const tenantId = 'e2e-tenant';

  test.use({ viewport: { width: 375, height: 667 } }); // Mobile-first

  test('should display incoming message and allow approval of AI draft via WebSocket', async ({ page, request }) => {
    // 1. Navigate to the new native omni-chat page
    await page.goto(`/api/v1/ui/omni-chat.html`);

    // We expect the UI to connect to the WebSocket and wait for messages
    await expect(page.locator('#connection-status')).toBeHidden({ timeout: 5000 });

    // 2. Simulate incoming message via our new native webhook
    const messageContent = 'Do you have vegan cupcakes for this Saturday?';
    const webhookRes = await request.post('/api/v1/omni-chat/webhook', {
      data: {
        channel: 'instagram_dm',
        sender_id: 'maya_insta',
        message: messageContent
      },
      headers: {
         // Mock authorization header if needed or let the server mock handle it
      }
    });
    expect(webhookRes.ok()).toBeTruthy();

    // 3. Verify message is pushed to the UI and visible
    const messageContext = page.locator(`.message-content:has-text("${messageContent}")`).first();
    await expect(messageContext).toBeVisible({ timeout: 10000 });

    // 4. The Ambassador Agent should have drafted a response automatically
    const draftContent = page.locator(`.draft-reply:has-text("The Ambassador regarding: ${messageContent}")`).first();
    await expect(draftContent).toBeVisible({ timeout: 10000 });

    // 5. Approve the draft
    const approveButton = page.locator('.approve-btn').first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Wait for the action to complete and show "Sent!"
    await expect(approveButton).toHaveText('Sent! ✅', { timeout: 5000 });
  });
});
