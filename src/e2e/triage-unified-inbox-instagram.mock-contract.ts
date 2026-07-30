import { test, expect } from './fixtures';

test.describe('Triage Inbox', () => {
  test('Loads the inbox properly', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('body')).toBeVisible();
  });
});
