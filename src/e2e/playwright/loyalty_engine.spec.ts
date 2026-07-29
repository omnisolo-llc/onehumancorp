import { test, expect } from '../fixtures';

test.describe('Loyalty Engine', () => {
  test('Loyalty Engine test', async ({ page }) => {
    await page.goto(`/loyalty`);
  });
});
