import { test, expect } from './fixtures';

test('simple test', async ({ page }) => {
  test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
