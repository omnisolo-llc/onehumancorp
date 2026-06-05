import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 1', () => {
  test('CUJ: Check extra 1', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});
