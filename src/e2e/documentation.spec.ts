import { test, expect } from './fixtures';

/**
 * NOTE: The full E2E run for this file requires `bazelisk test //src/e2e:playwright`
 * with the Docker mock servers running (PostgreSQL and Redis).
 */
test.describe('Documentation Features E2E', () => {

  test('should display Help Center and search for articles', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Check if initial articles load
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('My Store')).toBeVisible();

    // Perform search
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Finding Customers');

    // Expect filter to happen
    await expect(page.getByText('Getting Started')).not.toBeVisible();
    await expect(page.getByText('Finding Customers')).toBeVisible();
  });

  test('should navigate to specific article and read it', async ({ page }) => {
    await page.goto('/help/getting-started');

    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.getByText('Step 1: Tell us about your business')).toBeVisible();

    // Verify back navigation
    await page.getByRole('button', { name: /Back to Help Center/i }).click();
    await expect(page.url()).toContain('/help');
  });

  test('should display API Docs with Swagger UI mock/container', async ({ page }) => {
    await page.goto('/api-docs');

    await expect(page.getByText('Advanced:')).toBeVisible();
    await expect(page.getByText('This section is for developers directly integrating with our APIs. Not required for normal use.')).toBeVisible();
    // Swagger container class
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });

  test('should display Changelog entries', async ({ page }) => {
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();
    await expect(page.getByText('New Features')).toBeVisible();
  });

});
