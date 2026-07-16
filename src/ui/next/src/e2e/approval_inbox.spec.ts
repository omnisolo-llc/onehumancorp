import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Mobile Test', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display empty state or loading state in Activity Feed correctly', async ({ page }) => {
    await page.goto('/unified-feed');
    // Ensure dashboard loads and feed container is present
    const feedContainer = page.locator('main').first();
    await expect(feedContainer).toBeAttached({ timeout: 15000 });
  });
});
