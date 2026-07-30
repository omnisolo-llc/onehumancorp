import { test, expect } from './fixtures';

test.describe('Terminal Edge Sync', () => {
  test('Terminal loads without error', async ({ page }) => {
    await page.goto('/terminal');
    await expect(page.locator('body')).toBeVisible();
  });
});
