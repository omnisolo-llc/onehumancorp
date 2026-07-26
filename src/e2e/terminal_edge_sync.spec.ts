import { test, expect } from '@playwright/test';

test.describe('Terminal Edge Sync', () => {
  test('Loads terminal sync', async ({ page }) => {
    await page.goto('/terminal');
    await expect(page.locator('body')).toBeVisible();
  });
});
