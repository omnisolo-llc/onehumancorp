import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Chat Flow', () => {
  test.beforeEach(async ({ page }) => {
    const htmlPath = require('path').resolve('src/ui/tauri/src/ui/setup.html');
    await page.goto(`file://${htmlPath}`);
  });

  test('Verify chat bubble premium styling and flexbox layout', async ({ page }) => {
    // 1. Click 'Conversational Setup' button
    await page.locator('button:has-text("Conversational Setup")').click();

    // 2. Wait for the chat step to be visible
    const chatStep = page.locator('#step-chat');
    await expect(chatStep).toBeVisible();

    // 3. Verify the chat messages container has correct styles
    const chatMessages = page.locator('#chat-messages');
    await expect(chatMessages).toHaveCSS('display', 'flex');
    await expect(chatMessages).toHaveCSS('flex-direction', 'column');
    // // // await expect(chatMessages).toHaveCSS('border-radius', '16px');

    // 4. Verify initial assistant message styling
    const assistantMessage = page.locator('.chat-message.assistant').first();
    await expect(assistantMessage).toBeVisible();
    await expect(assistantMessage).toHaveCSS('align-self', 'flex-start');

    const assistantBubble = assistantMessage.locator('.chat-bubble');

    await expect(assistantBubble).toHaveCSS('border-bottom-left-radius', '4px');

    // 5. Send a chat message
    await page.fill('#chat-input', 'I run a custom cake shop');
    await page.click('#chat-send-btn');

    // 6. Verify user message styling
    const userMessage = page.locator('.chat-message.user').first();
    await expect(userMessage).toBeVisible();
    await expect(userMessage).toHaveCSS('align-self', 'flex-end');

    const userBubble = userMessage.locator('.chat-bubble');

    await expect(userBubble).toHaveCSS('border-bottom-right-radius', '4px');
    await expect(userBubble).toHaveCSS('background-color', 'rgb(0, 102, 255)');
    await expect(userBubble).toHaveCSS('color', 'rgb(255, 255, 255)');
  });
});
