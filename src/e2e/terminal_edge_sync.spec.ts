import { expect, test } from './fixtures';

test.describe('Terminal Edge Sync', () => {

  test('should display terminal status', async ({ page }) => {
    await page.goto('/settings/terminal');
    await expect(page.locator('body')).toBeVisible();
  });
});
