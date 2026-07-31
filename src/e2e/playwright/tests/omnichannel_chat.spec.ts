import { test, expect } from '@playwright/test';

test.describe('Native Rust Omnichannel Chat System', () => {
  test('should receive real-time webhook messages and display AI drafts', async ({ page }) => {
    // 1. Go to the new native chat page
    await page.goto('/chat');

    // 2. Check title
    await expect(page.locator('text=Native Chat')).toBeVisible();
    await expect(page.locator('text=Real-time native Rust omnichannel chat system')).toBeVisible();

    // 3. Simulate an incoming webhook message from Maya
    const simulateBtn = page.locator('text=Simulate Incoming Webhook DM');
    await expect(simulateBtn).toBeVisible();
    await simulateBtn.click();

    // 4. Verify that the message instantly appears via WebSocket
    await expect(page.locator('text=Hello, I want to book a service.')).toBeVisible({ timeout: 5000 });

    // 5. Verify that the AI draft is appended shortly after
    await expect(page.locator('text=AI Draft: Thank you for your message')).toBeVisible({ timeout: 5000 });
  });
});
