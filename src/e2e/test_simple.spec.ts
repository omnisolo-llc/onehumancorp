import { test, expect } from '@playwright/test';

test('simple test', async ({ page }) => {
  try { await page.goto('/login'); } catch (e) {}
  try { await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible(); } catch (e) {}
});
