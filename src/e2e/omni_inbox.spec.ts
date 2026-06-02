import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    await page.goto('/inbox');

    // Make sure we see the Unified Inbox header
    await expect(page.locator('h1', { hasText: 'Unified Inbox' })).toBeVisible();

    // Click Simulate Incoming Message
    await page.getByRole('button', { name: '🤖 Simulate' }).click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // We no longer have 'AI Replied' as a badge, but we have '✨ Ambassador Draft'
    // when a draft is ready for review. However, the simulation in the refactored code
    // uses the backend webhook which sends an actual drafted reply, then we poll the `/api/inbox/messages`.

    // In our UI, since we're using the backend webhook for simulation, the message will show up
    // after 1 second (as per the setTimeout in simulateIncomingMessage).
    await page.waitForTimeout(1500);

    // The webhook creates a new inbox_message row with draft_reply = '...'
    // Then it fetches threads and shows the draft bubble.
    await expect(page.getByText('Draft Ready for Review')).toBeVisible({ timeout: 5000 });

    // Click on the thread to open it
    await page.locator('h3', { hasText: 'Customer (sms)' }).click();

    // Verify reply content exists in the draft bubble
    // We expect the mock draft from the webhook
    await expect(page.getByText('Thank you for reaching out!')).toBeVisible();

    // Approve and Send
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // Verify it was sent
    await expect(page.locator('.bg-\\[\\#0066FF\\]')).toBeVisible(); // Sent message styling
  });
});
