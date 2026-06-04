import { test, expect } from './fixtures';

test('simple test', async ({ page }) => {
<<<<<<< HEAD
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
