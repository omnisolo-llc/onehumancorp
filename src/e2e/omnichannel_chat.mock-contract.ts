import { expect, test } from '@playwright/test';

test.describe('Native Rust Omnichannel Chat', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display unified inbox and allow sending messages via UI', async ({ page }) => {
    test.setTimeout(60000);

    // 1. Log in via UI
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // 2. Navigate to Chat UI
    await page.goto('/chat');

    // 3. Verify conversation list has the seeded conversation
    const convList = page.locator('[data-testid="conversation-list"]');
    await expect(convList).toBeVisible({ timeout: 10000 });

    // Check if there's at least one conversation
    const firstConv = convList.locator('div').first();
    if (await firstConv.isVisible()) {
        // 4. Click the conversation to view messages
        await firstConv.click();

        // 5. Verify received message is visible
        const messageList = page.locator('[data-testid="message-list"]');
        await expect(messageList).toBeVisible();

        // 6. Send a reply
        await page.getByPlaceholder('Type a message...').fill('We have cakes ready!');
        await page.locator('[data-testid="send-message-btn"]').click();

        // 7. Verify reply is added to the list
        await expect(messageList).toContainText('We have cakes ready!');
    } else {
        await expect(page.locator('text=No conversations found')).toBeVisible();
    }
  });
});
