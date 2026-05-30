import { test, expect } from './fixtures';

test.describe('Documentation Pages', () => {
  test('should display Help Center page', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Getting Started' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Finding Customers' })).toBeVisible();
  });

  test('should display Changelog page', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
  });

  test('should display API Docs page with Swagger UI', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced: This section is for developers')).toBeVisible();
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });
});
