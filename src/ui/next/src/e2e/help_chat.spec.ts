import { test, expect } from '@playwright/test';

test.describe('Help Chat Widget', () => {
    test('renders help chat and interacts', async ({ page }) => {
        await page.goto('/help');

        // Find and click the floating Ask anything button
        const chatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(chatButton).toBeVisible();
        await chatButton.click();

        // Wait for the chat to open and be visible
        const chatHeader = page.locator('#ai-chat-header');
        await expect(chatHeader).toBeVisible();

        // Check if the chat input is present
        const chatInput = page.locator('input[placeholder="Ask anything..."]');
        await expect(chatInput).toBeVisible();

        // Type a message and send it
        const testMessage = 'How do I add a product?';
        await chatInput.fill(testMessage);
        const sendButton = page.locator('button[aria-label="Send message"]');
        await expect(sendButton).toBeVisible();
        await sendButton.click();

        // Assert that the message appears in the chat
        const sentMessage = page.locator('div', { hasText: testMessage }).last();
        await expect(sentMessage).toBeVisible();

        // Close the chat
        const closeButton = page.locator('button[aria-label="Close help chat"]');
        await closeButton.click();
        await expect(chatHeader).not.toBeVisible();
    });
});
