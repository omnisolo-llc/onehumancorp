import { test, expect } from '../../../../e2e/fixtures';

test.describe('Dashboard Triage Edit', () => {
  test('Loads dashboard without error', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();
  });
});
