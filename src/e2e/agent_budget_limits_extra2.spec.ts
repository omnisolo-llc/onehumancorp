import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 2', () => {
  test('CUJ: Check extra 2', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});
