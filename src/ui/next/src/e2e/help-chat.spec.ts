import { test, expect } from '@playwright/test';

test.describe('Help Chat', () => {
    test('renders floating button and opens chat window', async ({ page }) => {
        await page.goto('/dashboard?test_chat=true');

        // Click the floating button to open the chat window
        const helpButton = page.locator('button[aria-label="Open help chat"]');
        await expect(helpButton).toBeVisible();
        await helpButton.click();

        // Check if chat window opens
        const chatHeader = page.locator('h3', { hasText: 'Ask AI Help' });
        await expect(chatHeader).toBeVisible();

        // Type and send a message
        const chatInput = page.getByPlaceholder('Ask me anything...');
        await chatInput.fill('How do I set up payments?');
        await page.locator('button[aria-label="Send message"]').click();

        // Check that user message is displayed
        await expect(page.locator('div', { hasText: 'How do I set up payments?' }).last()).toBeVisible();

        // Check that agent reply is displayed
        await expect(page.locator('div', { hasText: 'I am your AI Help Agent!' }).last()).toBeVisible();

        // Close chat
        await page.locator('button[aria-label="Close help chat"]').click();
        await expect(chatHeader).not.toBeVisible();
    });
});
