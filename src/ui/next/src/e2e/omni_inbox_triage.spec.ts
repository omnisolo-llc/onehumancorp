import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // 1. Intercept the inbox messages fetch to return our simulated data
    await page.route('**/api/ui/inbox/messages*', async (route) => {
      const json = [
        {
          id: 'msg_triage_1',
          source: 'Instagram DM',
          content: 'Hi Maya, do you have 2 vegan cakes for Saturday?',
          original_content: 'Hi Maya, do you have 2 vegan cakes for Saturday?',
          status: 'unread',
          sender_id: 'maya_customer_123',
          created_at: new Date().toISOString(),
          draft_reply: 'Yes! We have 2 available. Should I hold them for you? [Send & Deduct Inventory]'
        }
      ];
      await route.fulfill({ json });
    });

    // 2. Intercept the approvals fetch to simulate an active approval for this message
    await page.route('**/api/agents/approvals*', async (route) => {
      const json = {
        pending_approvals: [
          {
            id: 'app_triage_1',
            payload: JSON.stringify({
              inbox_message_id: 'msg_triage_1',
              action_type: 'Draft Reply'
            })
          }
        ]
      };
      await route.fulfill({ json });
    });

    // 3. Intercept the approve action
    let approveCalled = false;
    await page.route('**/api/agents/approvals/app_triage_1', async (route) => {
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

    // 4. Navigate to the inbox page
    await page.goto('/inbox');

    // 5. Assert the summary card is visible and displays the correct count
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible();
    await expect(summaryCard).toContainText('You have 1 unread lead.');

    // 6. Assert the message is visible in the list
    const messageButton = page.locator('button', { hasText: 'Instagram DM' });
    await expect(messageButton).toBeVisible();

    // Select the message (it might be auto-selected, but we click to be sure)
    await messageButton.click();

    // 7. Assert the draft reply with inventory deduction is shown
    await expect(page.locator('text="[Send & Deduct Inventory]"')).toBeVisible();

    // 8. Assert the special translucent action modal button is visible
    const approveButton = page.locator('button', { hasText: '✨ Approve & Send (Deduct Inventory)' });
    await expect(approveButton).toBeVisible();

    // 9. Click the button and verify action status
    await approveButton.click();
    await expect(page.locator('text="Draft approved and sent."')).toBeVisible();

    // Ensure the network call was made
    expect(approveCalled).toBe(true);
  });
});
