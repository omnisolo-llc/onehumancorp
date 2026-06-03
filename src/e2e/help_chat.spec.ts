import { test, expect } from './fixtures';

test.describe('HelpChat Widget E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate using the built-in path
    await page.goto('/');
  });

  test('should display Help Chat floating button', async ({ page }) => {
    const chatButton = page.locator('#global-chat-btn');
    await expect(chatButton).toBeVisible();
  });

  test('should open chat interface and display initial agent message', async ({ page }) => {
    const chatButton = page.locator('#global-chat-btn');
    await chatButton.click();

    const chatHeader = page.locator('#ai-chat-header', { hasText: 'Ask AI Help' });
    await expect(chatHeader).toBeVisible();

    const initialMessage = page.locator('text=Hi! I am your AI Support Agent. How can I help you grow your business today?');
    await expect(initialMessage).toBeVisible();
  });

  test('should allow typing a question and sending it', async ({ page }) => {
    const chatButton = page.locator('#global-chat-btn');
    await chatButton.click();

    const inputField = page.locator('#ai-chat-input');
    const sendButton = page.locator('button:has-text("Send")');

    await inputField.fill('How do I add a new product?');
    await expect(sendButton).toBeEnabled();
  });

  test('should display user message in the chat window upon submission', async ({ page }) => {
    const chatButton = page.locator('#global-chat-btn');
    await chatButton.click();

    const inputField = page.locator('#ai-chat-input');
    const sendButton = page.locator('button:has-text("Send")');

    await inputField.fill('How do I add a new product?');
    await sendButton.click();

    const userMessage = page.locator('div.chat-msg.user', { hasText: 'How do I add a new product?' });
    await expect(userMessage).toBeVisible();
  });

  test('should display agent reply after user submits a message', async ({ page }) => {
    const chatButton = page.locator('#global-chat-btn');
    await chatButton.click();

    const inputField = page.locator('#ai-chat-input');
    const sendButton = page.locator('button:has-text("Send")');

    await inputField.fill('Tell me about the dashboard features');
    await sendButton.click();

    // Verify user message appears
    await expect(page.locator('div.chat-msg.user', { hasText: 'Tell me about the dashboard features' })).toBeVisible();

    // Wait for agent reply (mocked in /api/chat endpoint to return "I am your AI Help Agent! ...")
    const agentReply = page.locator('text=I am your AI Help Agent!');
    await expect(agentReply).toBeVisible({ timeout: 10000 });
  });
});
