import { test, expect } from './fixtures';

test.describe('Changelog Page', () => {
  test('should display Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Interactive AI Store Builder:')).toBeVisible();
    await expect(page.getByText('Smart Tooltips:')).toBeVisible();
  });
});
