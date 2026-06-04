import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 3', () => {
  test('CUJ: Check extra 3', async ({ page }) => {
<<<<<<< HEAD
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});
