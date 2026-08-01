import { test, expect } from '../fixtures';

test.describe('Loyalty & Rewards Engine', () => {

  test('Should navigate to loyalty page', async ({ page }) => {
    await page.goto('/loyalty');
    await expect(page.locator('body')).toBeVisible();
  });
});
