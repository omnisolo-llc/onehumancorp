import { test, expect } from './fixtures';

test.describe("Wrapped Auto", () => {
  test('simple test', async ({ page }) => {
});
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
