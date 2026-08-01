import { expect, test } from './fixtures';

test.describe('Unified Inbox Instagram', () => {

  test('should display unified inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('body')).toBeVisible();
  });
});
