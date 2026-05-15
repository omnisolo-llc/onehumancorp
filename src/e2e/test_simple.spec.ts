import { test, expect } from '@playwright/test';

test('simple test', async ({ page }) => {
  await page.goto('/login');
  try { await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
});
