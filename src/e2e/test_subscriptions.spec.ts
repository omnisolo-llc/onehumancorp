import { test, expect } from './fixtures';

test.describe('Subscriptions', () => {
  test('Subscriptions test', async ({ page }) => {
    await page.goto(`/subscriptions`);
  });
});
