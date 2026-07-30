import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox', () => {
  test('Loads omni inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('body')).toBeVisible();
  });
});
