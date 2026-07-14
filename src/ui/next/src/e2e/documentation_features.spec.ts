import { test, expect } from '../../../../e2e/fixtures';

test.describe('Documentation Features', () => {

  test('Changelog page loads correctly', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByTestId('changelog-title')).toBeVisible();
    await expect(page.getByText('Release Notes & Changelog')).toBeVisible();
  });

  test('API Docs page loads correctly', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByTestId('api-docs-title')).toBeVisible();
    await expect(page.getByText('Advanced:')).toBeVisible();
  });
});
