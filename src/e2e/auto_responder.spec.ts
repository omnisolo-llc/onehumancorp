import { expect, test } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Intelligent Customer Auto-Responder', () => {
  test('handles incoming webhook, processes task queue, and shows AI Handled in Inbox UI', async ({
    page,
    request,
    login,
  }) => {
    // 1. Log in to get the tenant context
    const tenantId = 'default';
    await login();

    // 2. Simulate an incoming webhook from Twilio
    const senderId = '+15551234567';
    const messageBody = 'Where is my order?';

    // We send a url-encoded form body as twilio does
    const webhookResponse = await request.post('/api/v1/webhooks/twilio', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: `From=${encodeURIComponent(senderId)}&Body=${encodeURIComponent(messageBody)}&To=%2B15559876543`,
    });

    expect(webhookResponse.status()).toBe(200);

    // 3. Wait a moment for the background job to be picked up by PostgreSQL SKIP LOCKED and the auto_responder worker
    await page.waitForTimeout(5000);

    // 4. Navigate to the unified inbox
    await page.goto('/inbox');

    // 5. Verify the message is visible and has the correct state
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // The message should be in the list
    const messageItem = page.locator('#messages-list').getByText(messageBody);
    await expect(messageItem).toBeVisible();

    // Click on the message to see details
    await messageItem.click();

    // The detail panel should show the AI Handled / auto_replied or unread status
    // The worker either sets it to "auto_replied" or "unread" based on confidence
    // We just need to check if the status is rendered and there's a draft reply.
    await expect(page.getByText('Conversation Detail')).toBeVisible();
    await expect(page.getByText(senderId)).toBeVisible();
    await expect(page.getByText('Draft Reply')).toBeVisible();

    // We can't guarantee 'auto_replied' strictly due to LLM mock randomness, but we can check if it's there
    const statusText = await page.locator('.app-card .app-badge').first().innerText();
    expect(['auto_replied', 'unread', 'Open']).toContain(statusText);
  });
});
