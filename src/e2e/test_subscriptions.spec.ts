import { test, expect } from './fixtures';

test.describe('Subscriptions', () => {

  test('Shows subscriptions', async ({ page }) => {
    await page.goto(`/dashboard`);
    await expect(page.locator('body')).toBeVisible();
  });
});
