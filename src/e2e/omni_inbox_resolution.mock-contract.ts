import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat Inbox CUJ', () => {
  const tenantId = `tenant-${Math.random().toString(36).substring(7)}`;
  const conversationId = `conv-${Math.random().toString(36).substring(7)}`;

  test('Owner can view conversations and reply', async ({ page }) => {
    await page.route('**/api/v1/chat-inbox/conversations', async (route) => {
      await route.fulfill({
        status: 200,
        json: [
          {
            id: conversationId,
            status: 'open',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
            contact_name: 'Priya the Customer'
          }
        ]
      });
    });

    await page.route(`**/api/v1/chat-inbox/conversations/${conversationId}/messages`, async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          json: [
            {
              id: 'msg-1',
              sender_type: 'contact',
              content: 'Hi, is my order ready?',
              created_at: new Date().toISOString()
            }
          ]
        });
      } else if (route.request().method() === 'POST') {
        const body = JSON.parse(route.request().postData() || '{}');
        await route.fulfill({
          status: 201,
          json: {
            id: 'msg-2',
            sender_type: 'agent',
            content: body.content,
            created_at: new Date().toISOString()
          }
        });
      }
    });

    await page.goto('/chat');

    // Select the conversation
    const conversationBtn = page.locator(`[data-testid="conversation-${conversationId}"]`);
    await expect(conversationBtn).toBeVisible();
    await conversationBtn.click();

    // Assert message is displayed
    await expect(page.locator('text="Hi, is my order ready?"')).toBeVisible();

    // Type a reply
    const input = page.locator('[data-testid="chat-input"]');
    await input.fill('Yes, it is ready for pickup!');

    // Send reply
    const sendBtn = page.locator('[data-testid="chat-send"]');
    await sendBtn.click();

    // Assert the new message is displayed
    await expect(page.locator('text="Yes, it is ready for pickup!"')).toBeVisible();
  });
});
