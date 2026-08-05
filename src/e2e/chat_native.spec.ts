import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat CUJ', () => {
  // Simulating Carlos receiving an SMS lead, seeing it in the inbox, reviewing the AI draft, and sending it.

  test('receives message, views AI draft, and sends', async ({ request, page }) => {
    // 1. Simulate webhook ingress (SMS lead)
    const payload = {
      tenant_id: '00000000-0000-0000-0000-000000000001',
      source: 'sms',
      sender_id: '00000000-0000-0000-0000-000000000002',
      message: 'Hi, I need a repair estimate for my roof.'
    };

    // We use a mock URL for this test, but normally request.post hits the local stack
    // if configured via baseURL. If it is not, we use absolute.
    // For this e2e, we skip real network and assert UI.
    await page.route('**/api/v1/chat/conversations', async (route) => {
       await route.fulfill({
           json: [{
             id: '123',
             contact_id: 'Unknown Contact',
             status: 'open',
             updated_at: new Date().toISOString()
           }]
       });
    });

    await page.route('**/api/v1/chat/conversations/123/messages', async (route) => {
        if (route.request().method() === 'POST') {
             await route.fulfill({
                 json: {
                     id: 'msg-2',
                     content: 'Thank you! We will get back to you shortly.',
                     sender_type: 'agent',
                     status: 'sent',
                     created_at: new Date().toISOString()
                 }
             });
             return;
        }
        await route.fulfill({
            json: [{
              id: 'msg-1',
              content: 'Thank you! We will get back to you shortly.',
              sender_type: 'bot',
              status: 'draft',
              created_at: new Date().toISOString()
            }]
        });
    });

    // 2. Login to UI
    // In our hermetic tests, direct navigation works when mocked
    await page.goto('http://localhost:3000/inbox');

    // 3. Verify Inbox loads
    await expect(page.locator('text=Native Omnichannel Threads')).toBeVisible();

    // 4. Verify thread exists
    const threadList = page.locator('text=Active Threads');
    await expect(threadList).toBeVisible();

    // 5. Check if the AI draft is visible in the thread
    const activeThreads = page.locator('button', { hasText: 'Unknown Contact' }).first();
    await expect(activeThreads).toBeVisible();
    await activeThreads.click();

    // Verify AI Suggested Reply
    await expect(page.locator('text=✨ AI Suggested Reply')).toBeVisible();

    // 6. Tap "Send Draft"
    const sendDraftBtn = page.locator('button', { hasText: 'Send Draft' });
    await expect(sendDraftBtn).toBeVisible();
    await sendDraftBtn.click();

    // Verify draft is removed and sent message appears
    await expect(page.locator('text=✨ AI Suggested Reply')).toBeHidden();
    await expect(page.locator('text=Thank you! We will get back to you shortly.')).toBeVisible();
  });
});
