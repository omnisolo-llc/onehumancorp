import { test, expect } from './fixtures';

test.describe('Documentation & Help Center', () => {
  test('should load Help Center and display search functionality', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByPlaceholder('Search for help articles...')).toBeVisible();

    // Verify articles are rendered
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('My Store')).toBeVisible();

    // Test search filtering
    await page.getByPlaceholder('Search for help articles...').fill('payments');
    await expect(page.getByText('Getting Paid')).toBeVisible();
    await expect(page.getByText('My Store')).not.toBeVisible();
  });

  test('should load individual Help Article', async ({ page }) => {
    await page.goto('/help/getting-started');
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Back to Help Center' })).toBeVisible();
  });

  test('should load Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
  });

  test('should load API Docs', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
  });
});
