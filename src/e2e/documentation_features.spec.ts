import { test, expect } from './fixtures';

test.describe('Documentation Features', () => {
  test('should display changelog page', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();
    await expect(page.getByRole('link', { name: /Read the full technical changelog/i })).toBeVisible();
  });

  test('should display api-docs page', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText(/Advanced:/)).toBeVisible();
    await expect(page.getByText(/This section is for developers/)).toBeVisible();
  });

  test('should open help center', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
  });

  test('should search and find help articles', async ({ page }) => {
    await page.goto('/help');
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Started');
    await expect(page.getByText('Learn how to easily set up your store')).toBeVisible();
  });

  test('should open specific help article', async ({ page }) => {
    await page.goto('/help/getting-started');
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.getByText('Step 1: Tell us about your business')).toBeVisible();
  });
});
