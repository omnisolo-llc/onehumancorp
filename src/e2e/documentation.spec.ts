import { test, expect } from './fixtures';

test.describe('Documentation Features', () => {
  test('Help Center search and navigation', async ({ page }) => {
    // Navigate to Help Center directly instead of through navigation since
    // it handles login and seeding via fixtures.
    await page.goto('/help');

    // Verify Help Center is loaded
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Test search functionality
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('Getting Started');

    // Verify search results filter correctly
    await expect(page.getByRole('heading', { name: 'Getting Started' }).first()).toBeVisible();

    // Test navigation to article details
    const articleLink = page.getByRole('link', { name: 'Getting Started Learn how to' });
    await articleLink.click();

    // Verify article page is loaded
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Back to Help Center' })).toBeVisible();
  });

  test('Changelog loads correctly', async ({ page }) => {
    // Navigate to Changelog
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();

    // Verify at least one version block or empty state is shown
    // Using a generic selector as the data is fetched from the backend
    const versionHeaders = page.getByRole('heading').filter({ hasText: /^\d+\.\d+/ });
    const emptyState = page.getByText('No changelog available.');

    await expect(versionHeaders.first().or(emptyState)).toBeVisible();
  });

  test('API Docs loads correctly', async ({ page }) => {
    // Navigate to API Docs
    await page.goto('/api-docs');

    // Verify API Docs page is loaded (Advanced section warning)
    await expect(page.getByText('This section is for developers directly integrating with our APIs')).toBeVisible();

    // Verify Swagger UI container exists
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });
});
