import { test, expect } from './fixtures';

test('simple test', async ({ page }) => {
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('http://localhost:3000/login').catch(() => {});
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
