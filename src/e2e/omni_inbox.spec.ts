import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('displays the database-backed inbox experience and processes omni messages', async ({ page, request }) => {
    // 1. Simulate an incoming webhook payload
    const senderId = `user_${randomUUID()}@example.com`;
    const messageContent = 'Hello, do you fix sinks?';

    // Instead of mocking the network in E2E, we'll actually fire a test webhook
    // against the local running backend.
    const res = await request.post('/api/inbox/webhook', {
      data: {
        source: 'email',
        sender: senderId,
        content: messageContent,
        tenant_id: 'default'
      }
    });

    // Check that our backend accepted it
    expect(res.ok()).toBeTruthy();

    // 2. Navigate to the UI and ensure the message appears
    await page.goto('/inbox');

    // No explicit mock delays, wait for real network request and rendering
    const messageCard = page.locator('.inbox-message', { hasText: messageContent });
    await expect(messageCard).toBeVisible({ timeout: 10000 });

    // 3. Verify the AI auto-reply draft feature
    await messageCard.click();
    const draftText = page.locator('.draft-reply-content');

    // The backend AI should have prepared a draft reply. We wait for it.
    await expect(draftText).toBeVisible({ timeout: 15000 });
    await expect(draftText).toContainText('sink');
  });
});
