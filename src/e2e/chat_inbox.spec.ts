import { test, expect } from '@playwright/test';

// @no-substitution
test.describe('Omnichannel Chat Inbox E2E', () => {
  test('User can load the inbox and see the empty state', async ({ page }) => {
    // Navigate to the inbox UI with a specific tenant
    await page.goto('/api/v1/ui/inbox.html?tenant_id=e2e-tenant');

    // Wait for the app to initialize
    await page.waitForSelector('.header h1');
    await expect(page.locator('.header h1')).toHaveText('Inbox');

    // The conversation list should eventually show "No conversations yet" since it's empty
    await page.waitForSelector('.conversation-list .empty-state');
    await expect(page.locator('.conversation-list .empty-state')).toContainText('No conversations yet');

    // Check if the main view prompts the user to select a chat
    await expect(page.locator('#messages-container .empty-state')).toContainText('Select a conversation to start chatting');
  });

  test('Inbox is mobile responsive at 375px', async ({ page }) => {
    // Set viewport to mobile size
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/api/v1/ui/inbox.html?tenant_id=e2e-tenant');
    await page.waitForSelector('.sidebar');

    // On mobile, the sidebar should take up the full screen or be toggleable
    const sidebarBoundingBox = await page.locator('.sidebar').boundingBox();
    expect(sidebarBoundingBox?.width).toBe(375);
  });
});
