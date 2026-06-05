import { test, expect } from './fixtures';

test('funding engine mock dashboard UI check', async ({ page }) => {
  test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
