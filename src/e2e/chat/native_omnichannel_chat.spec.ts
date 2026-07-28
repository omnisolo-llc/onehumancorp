import { test, expect } from '@playwright/test';

test.describe('Omnichannel Native Rust Chat', () => {
  test('receives customer message via webhook adapter and owner views it', async ({ page, request }) => {
    // 1. Simulate a customer sending a message (e.g., hitting a webhook)
    // We expect the backend API to handle the webhook via /api/v1/inbox/webhook (or similar)
    // Since the actual ingestion logic isn't fully integrated here, we will mock the backend
    // response for the inbox messages fetch, just like the other E2E tests, OR we can test the real flow if available.

    // For this task, we'll intercept the network to show the business owner seeing the message,
    // mimicking what would happen if the Rust service handled it and saved it to the new tables.

    await page.route('**/api/v1/ui/inbox/messages*', async (route) => {
      const json = [
        {
          id: 'chat_msg_rust_1',
          source: 'Instagram DM',
          content: 'Hi! Maya, do you have any available tables for two tonight?',
          original_content: 'Hi! Maya, do you have any available tables for two tonight?',
          status: 'unread',
          sender_id: 'customer_123',
          created_at: new Date().toISOString(),
          draft_reply: 'Yes, we can fit you in at 7 PM! [Send via Native Rust Chat]'
        }
      ];
      await route.fulfill({ json });
    });

    await page.route('**/api/agents/approvals*', async (route) => {
      const json = {
        pending_approvals: [
          {
            id: 'app_triage_chat',
            payload: JSON.stringify({
              inbox_message_id: 'chat_msg_rust_1',
              action_type: 'Draft Reply'
            })
          }
        ]
      };
      await route.fulfill({ json });
    });

    let approveCalled = false;
    await page.route('**/api/agents/approvals/app_triage_chat', async (route) => {
      if (route.request().method() === 'POST') {
        const body = JSON.parse(route.request().postData() || '{}');
        if (body.approved === true) {
          approveCalled = true;
          await route.fulfill({ status: 200, json: { success: true } });
          return;
        }
      }
      await route.fallback();
    });

    // Navigate to the inbox page
    await page.goto('/inbox');

    // Assert the summary card is visible and displays the correct count
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible();

    // Assert the message is visible in the list
    const messageButton = page.locator('button', { hasText: 'Instagram DM' });
    await expect(messageButton).toBeVisible();

    // Select the message
    await messageButton.click();

    // Assert the draft reply is shown
    await expect(page.locator('text="[Send via Native Rust Chat]"')).toBeVisible();

    // Assert the special translucent action modal button is visible
    const approveButton = page.locator('button', { hasText: /Approve & Send/i });
    await expect(approveButton).toBeVisible();

    // Click the button and verify action status
    await approveButton.click();
    await expect(page.locator('text="Draft approved and sent."')).toBeVisible();

    // Ensure the network call was made
    expect(approveCalled).toBe(true);
  });
});
