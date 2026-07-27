import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat System', () => {
  test('End-to-End Chat Flow', async ({ page }) => {
    // Navigate to the chat page
    await page.goto('/omnichannel/chat');

    // Wait for the UI to load
    await expect(page.locator('h1').filter({ hasText: 'Unified Inbox' })).toBeVisible();

    // Create a new conversation
    await page.click('data-testid=new-conversation-btn');

    // Wait for the conversation view to appear
    await expect(page.locator('h2').filter({ hasText: 'Conversation' })).toBeVisible();

    // Simulate sending a contact message
    await page.fill('data-testid=omni-chat-input', 'Hello, I need help with my cake order!');
    await page.click('data-testid=omni-chat-simulate');

    // Message should appear in chat
    await expect(page.getByText('Hello, I need help with my cake order!')).toBeVisible();

    // The AI worker should generate a draft (we wait a bit for backend processing)
    // Wait for AI Draft pill to appear
    await expect(page.getByText('AI Draft')).toBeVisible({ timeout: 15000 });

    // Approve the draft
    await page.click('data-testid=approve-draft-btn');

    // AI Draft pill should disappear for that message (or the new message is sent)
    await expect(page.getByText('AI Draft')).not.toBeVisible();

    // Send a manual reply
    await page.fill('data-testid=omni-chat-input', 'Thank you!');
    await page.click('data-testid=omni-chat-send');

    // Go back to inbox
    await page.click('data-testid=omni-back-btn');
    await expect(page.locator('h1').filter({ hasText: 'Unified Inbox' })).toBeVisible();
  });
});
