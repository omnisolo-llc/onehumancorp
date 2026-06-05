import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 3', () => {
  test('CUJ: Check extra 3', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});
