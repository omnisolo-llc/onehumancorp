import { test, expect } from './fixtures';

test.describe('HelpChat Widget E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard?test_chat=true');
  });

  test('should display Help Chat floating button', async ({ page }) => {
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
  });

  test('should open chat interface and display initial agent message', async ({ page }) => {
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await chatButton.click({ force: true });

    const chatHeader = page.locator('h3', { hasText: 'Ask AI Help' });
    await expect(chatHeader).toBeVisible();

    const initialMessage = page.locator('text=Need help setting up your store');
    await expect(initialMessage).toBeVisible();
  });

  test('should enable send button when typing a question', async ({ page }) => {
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await chatButton.click({ force: true });

    const inputField = page.locator('input[placeholder="Ask anything..."]');
    const sendButton = page.locator('button[aria-label="Send message"]');

    await expect(sendButton).toBeDisabled();
    await inputField.fill('How do I add a new product?');
    await expect(sendButton).toBeEnabled();
  });

  test('should display user message in the chat window upon submission', async ({ page }) => {
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await chatButton.click({ force: true });

    const inputField = page.locator('input[placeholder="Ask anything..."]');
    const sendButton = page.locator('button[aria-label="Send message"]');

    await inputField.fill('How do I add a new product?');
    await sendButton.click();

    const userMessage = page.locator('text=How do I add a new product?');
    await expect(userMessage).toBeVisible();
  });

  test('should display agent reply after user submits a message', async ({ page }) => {
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await chatButton.click({ force: true });

    const inputField = page.locator('input[placeholder="Ask anything..."]');
    const sendButton = page.locator('button[aria-label="Send message"]');

    await inputField.fill('Tell me about the dashboard features');
    await sendButton.click();

    // Verify user message appears
    await expect(page.locator('text=Tell me about the dashboard features')).toBeVisible();

    // Wait for agent reply (mocked in /api/v1/chat endpoint to return "I am your AI Help Agent! ...")
    const agentReply = page.locator('text=I am your AI Help Agent!');
    await expect(agentReply).toBeVisible({ timeout: 10000 });
  });
});
