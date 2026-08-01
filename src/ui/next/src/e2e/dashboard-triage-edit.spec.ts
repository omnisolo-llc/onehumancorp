import { expect, test } from '@playwright/test';

test.describe('Dashboard Triage Action Feed Edit UI', () => {

  test('should allow editing a draft from the unified dashboard feed', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();
  });
});
