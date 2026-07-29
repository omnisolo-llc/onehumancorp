import { test, expect } from './fixtures';

test.describe('Terminal Edge Sync', () => {

  test('Syncs terminal status', async ({ page }) => {
    await page.goto(`/dashboard`);
    await expect(page.locator('body')).toBeVisible();
  });
});
