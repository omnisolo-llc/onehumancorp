import { test, expect } from '@playwright/test';

test.describe('HelpChat Component', () => {
    test('renders floating button, opens chat, sends message, and closes chat', async ({ page }) => {
        // Mock the chat API to avoid LLM calls or flakes
        await page.route('/api/chat', async route => {
            const json = {
                reply: "I am a mocked AI Help Agent response.",
                link: { url: "/help", title: "Read the full article →" }
            };
            await route.fulfill({ json });
        });

        // Use the dashboard or layout that includes the HelpChat widget
        // The URL needs `test_chat=true` to bypass the E2E block on the HelpChat rendering.
        await page.goto('/dashboard?test_chat=true');

        // Locate the floating help chat button
        const openChatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(openChatButton).toBeVisible();

        // Open the chat
        await openChatButton.click();

        // Verify chat interface is open
        const chatHeader = page.locator('#ai-chat-header');
        await expect(chatHeader).toBeVisible();
        await expect(chatHeader.locator('h3', { hasText: 'Ask AI Help' })).toBeVisible();

        // Verify initial message from agent
        await expect(page.locator('div', { hasText: "Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?" }).first()).toBeVisible();

        // Type a message
        const inputField = page.locator('input[placeholder="Ask me anything..."]');
        await inputField.fill('How do I add a new product?');

        // Send the message
        const sendButton = page.locator('button[aria-label="Send message"]');
        await sendButton.click();

        // Wait for the mocked response
        await expect(page.locator('div', { hasText: 'I am a mocked AI Help Agent response.' }).first()).toBeVisible();
        await expect(page.locator('a', { hasText: 'Read the full article →' })).toBeVisible();

        // Close the chat
        const closeButton = page.locator('button[aria-label="Close help chat"]');
        await closeButton.click();

        // Verify chat interface is closed
        await expect(chatHeader).not.toBeVisible();
    });
});
