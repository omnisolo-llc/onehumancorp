import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 2', () => {
  test('CUJ: Check extra 2', async ({ page }) => {
    await page.goto('/assistant');
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' }).first()).toBeVisible();
  });
});
