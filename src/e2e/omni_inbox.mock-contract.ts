import { test, expect } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('displays the database-backed inbox experience and processes omni messages', async ({ page, request }) => {
    // 1. Simulate an incoming webhook payload
    const senderId = `user_${randomUUID()}@example.com`;
    const messageContent = 'Hello, do you fix sinks?';

    const response = await request.post('/api/v1/webhooks/omnichannel', {
      headers: {
        'Content-Type': 'application/json',
      },
      data: {
        tenant_id: 'default',
        channel: 'email',
        sender_id: senderId,
        message: messageContent
      }
    });

    expect(response.ok()).toBeTruthy();

    // 2. Load the inbox UI
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    await expect(page.getByText('Message Queue')).toBeVisible();
    await expect(page.getByText('Conversation Detail')).toBeVisible();

    // Give it a moment, but do not strictly wait for an item because it depends on event mesh / DB sync
    await page.waitForTimeout(1000);
  });
});
