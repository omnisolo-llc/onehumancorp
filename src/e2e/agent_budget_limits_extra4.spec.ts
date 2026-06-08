import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 4', () => {
  test('CUJ: Check extra 4', async ({ page }) => {
    await page.goto('/assistant');
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' }).first()).toBeVisible();
  });
});
