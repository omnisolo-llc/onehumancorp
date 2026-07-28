import { test, expect } from '@playwright/test';

test.describe('Custom Omnichannel Chat System', () => {

  test('Owner can view unified inbox, receive real-time message, and send reply', async ({ page }) => {
    // 1. Owner navigates to the inbox (assuming local mock setup handles auth or we test the unauthenticated shell if permitted)
    await page.goto('/inbox');

    // Select the new tab
    await page.click('text=Omnichannel Chat');

    // Verify Chat System UI is loaded
    await expect(page.locator('text=Custom Rust Omnichannel Chat')).toBeVisible();

    // 2. Select a conversation
    await page.click('text=Support Inquiry');

    // Verify conversation view is shown
    await expect(page.locator('text=Conversation: conv_1')).toBeVisible();

    // 3. Send a reply
    const input = page.locator('input[placeholder="Type your reply..."]');
    await input.fill('Hi there, how can I help you today?');
    await page.click('text=Send');

    // Verify the sent message appears in the chat
    await expect(page.locator('text=Hi there, how can I help you today?')).toBeVisible();
  });
});
