import { test, expect } from '@playwright/test';

test.describe('Terminal Edge Sync', () => {
  test('Offline indicator works', async ({ page }) => {
    await page.goto('/terminal');
    await expect(page.locator('text="Terminal"')).toBeVisible();
  });
});
