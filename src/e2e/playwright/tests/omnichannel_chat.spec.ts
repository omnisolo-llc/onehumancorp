import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat unified feed', () => {
  test('Owner can see active conversations', async ({ page }) => {
    // Navigate to the unified inbox feed
    await page.goto('/inbox');

    // Wait for the feed to load
    await expect(page.locator('.unified-feed')).toBeVisible();

    // Verify conversation view renders correctly on mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // Assuming we have mock data seeded through the API
    const conversation = page.locator('.conversation-item').first();
    await expect(conversation).toBeVisible();

    // Click to view the conversation
    await conversation.click();

    // Verify the conversation view is active
    await expect(page.locator('.conversation-view')).toBeVisible();

    // Verify channel icon is shown
    await expect(page.locator('.channel-icon')).toBeVisible();

    // Check for native mobile keyboard support hint
    await expect(page.locator('.chat-input')).toBeVisible();
  });
});
