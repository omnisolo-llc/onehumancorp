import { test, expect } from './fixtures';

test.describe('Terminal Edge Sync', () => {
  test('Terminal Edge Sync test', async ({ page }) => {
    await page.goto(`/terminal`);
  });
});
