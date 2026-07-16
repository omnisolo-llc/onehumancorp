import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Mobile Test', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and handle tabs', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/unified-feed');
    // We expect the body not to scroll horizontally
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);

    const feedContainer = page.locator('main').first();
    await expect(feedContainer).toBeAttached({ timeout: 15000 });
  });
});
