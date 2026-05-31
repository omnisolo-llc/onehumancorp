import { test, expect } from './fixtures';

test.describe('Help Chat UI', () => {
  test('should open help chat and send a message', async ({ page }) => {
    await page.goto('/');

    // Wait for floating button and click
    const chatButton = page.getByRole('button', { name: 'Ask anything' });
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Verify chat UI opened
    const chatContainer = page.locator('.animate-slide-up-chat');
    await expect(chatContainer).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Help Agent' })).toBeVisible();

    // Type a message
    const chatInput = page.getByPlaceholder('Ask me anything...');
    await chatInput.fill('How do I add a new product?');

    // Send message
    await page.getByRole('button', { name: 'Send message' }).click();

    // Verify user message appeared
    await expect(page.getByText('How do I add a new product?')).toBeVisible();

    // Verify AI reply
    await expect(page.getByText(/I am your AI Help Agent!/)).toBeVisible();

    // Ensure the link is there
    await expect(page.getByRole('link', { name: 'Read the full article →' })).toBeVisible();
  });
});
