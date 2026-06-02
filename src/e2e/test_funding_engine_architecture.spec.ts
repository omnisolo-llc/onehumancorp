import { test, expect } from './fixtures';

test('funding engine mock dashboard UI check', async ({ page }) => {
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
