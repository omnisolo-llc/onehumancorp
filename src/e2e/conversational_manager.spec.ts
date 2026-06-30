import { test, expect } from './fixtures';

test.describe('Conversational AI Store Manager', () => {
  test('should load the living mobile editor and allow sending a message', async ({ page, adminUser, loginAs }) => {
    // Navigate using the login fixture to ensure auth token/cookies are present.
    await loginAs(page, adminUser);
    await page.goto('/ui/conversational-manager.html');

    // Verify initial render
    await expect(page).toHaveTitle('Living Mobile Editor');

    // Verify chat container is visible
    const chatContainer = page.locator('#chat-container');
    await expect(chatContainer).toBeVisible();

    // Type a message in the input
    const messageInput = page.locator('#message-input');
    await messageInput.fill('Update Business Hours');

    // Verify send button becomes enabled
    const sendBtn = page.locator('#send-btn');
    await expect(sendBtn).toBeEnabled();

    // Send the message
    await sendBtn.click();

    // Verify user message appears in chat container
    const userMessage = chatContainer.locator('.message.user').last();
    await expect(userMessage).toBeVisible();
    await expect(userMessage).toHaveText('Update Business Hours');

    // Verify input is cleared
    await expect(messageInput).toHaveValue('');

    // Wait for the backend response
    const agentMessage = chatContainer.locator('.message.agent').last();
    await expect(agentMessage).toBeVisible({ timeout: 10000 });

    // In a fully hermetic environment, we will get the hardcoded response from `handle_conversational_chat`
    // "I can help you update your store settings..."
    // We can just verify it is generally visible
    await expect(agentMessage).toContainText(/update|help/i);

    // Verify an action card is presented for updating hours
    const actionCard = chatContainer.locator('.action-card').last();
    await expect(actionCard).toBeVisible();
    await expect(actionCard.locator('h3')).toHaveText('Update Business Hours');
    await expect(actionCard.locator('p')).toHaveText('Change Saturday hours to 10AM - 2PM');

    // Test the cancel button
    const cancelBtn = actionCard.locator('.btn-cancel');
    await cancelBtn.click();

    // Verify it appends cancelled message
    const cancelledMsg = chatContainer.locator('.message.agent').last();
    await expect(cancelledMsg).toHaveText('Action cancelled.');
  });
});
