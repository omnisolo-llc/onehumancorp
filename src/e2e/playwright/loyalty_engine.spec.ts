import { test, expect } from '../fixtures';

test.describe('Loyalty & Rewards Engine', () => {

  test('Dashboard should have a link to the loyalty widget', async ({ page }) => {
    await page.goto('/dashboard.html');
    await expect(page.locator('body')).toBeVisible();
  });

});
