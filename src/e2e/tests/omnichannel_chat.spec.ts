import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat System Mobile View', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile viewport

  test.beforeEach(async ({ page }) => {
    // Navigate to the inbox page
    await page.goto('/inbox');
  });

  test('should display the inbox tab bar icon', async ({ page }) => {
    const inboxTab = page.locator('nav').locator('text=Inbox');
    await expect(inboxTab).toBeVisible();
  });

  test('should show a unified list of messages with unread indicators', async ({ page }) => {
    const messageList = page.locator('.conversation-list');
    await expect(messageList).toBeVisible();

    // Check for at least one unread indicator
    const unreadIndicator = page.locator('.unread-badge').first();
    await expect(unreadIndicator).toBeVisible();
  });

  test('should display AI summaries in the conversation list', async ({ page }) => {
    const aiSummary = page.locator('.ai-summary').first();
    await expect(aiSummary).toBeVisible();
  });

  test('should open a chat screen with message bubbles and input area', async ({ page }) => {
    await page.locator('.conversation-item').first().click();

    const chatScreen = page.locator('.chat-screen');
    await expect(chatScreen).toBeVisible();

    const messageBubble = page.locator('.message-bubble').first();
    await expect(messageBubble).toBeVisible();

    const inputArea = page.locator('textarea[placeholder="Type a message..."]');
    await expect(inputArea).toBeVisible();

    const attachmentButton = page.locator('button[aria-label="Attach file"]');
    await expect(attachmentButton).toBeVisible();
  });

  test('should display inline AI suggested replies and allow sending', async ({ page }) => {
    await page.locator('.conversation-item').first().click();

    const aiSuggestedReply = page.locator('.ai-suggested-reply').first();
    await expect(aiSuggestedReply).toBeVisible();

    const replyText = await aiSuggestedReply.innerText();
    await aiSuggestedReply.click();

    const inputArea = page.locator('textarea[placeholder="Type a message..."]');
    await expect(inputArea).toHaveValue(replyText);

    const sendButton = page.locator('button[aria-label="Send message"]');
    await sendButton.click();

    // Check if the message is added to the bubbles
    const lastMessage = page.locator('.message-bubble').last();
    await expect(lastMessage).toContainText(replyText);
  });
});
