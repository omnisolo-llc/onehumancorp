import { test, expect } from '@playwright/test';

test('simple test', async ({ page }) => {
  await page.goto('/login');
  await expect(page.locator('h1').first()).toBeVisible();
});
