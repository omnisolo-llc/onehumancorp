import { test, expect } from '@playwright/test';

test.describe('Omnichannel Native Chat', () => {
  test('renders unified inbox layout correctly on mobile and desktop', async ({ page }) => {
    await page.goto('/chat');

    // Check if the route is defined and list renders
    await expect(page.locator('text=Unified Inbox')).toBeVisible();

    const isMobile = await page.evaluate(() => window.innerWidth < 768);
    if (isMobile) {
      await expect(page.locator('text=Select a conversation to start chatting')).toBeHidden();
    } else {
      await expect(page.locator('text=Select a conversation to start chatting')).toBeVisible();
    }
  });
});
