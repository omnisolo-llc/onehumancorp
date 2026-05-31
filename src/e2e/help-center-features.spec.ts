import { test, expect } from './fixtures';

test.describe('Help Center Documentation', () => {

  test('help center page loads and is searchable', async ({ page }) => {
    // A business owner needs help with their store and navigates to the help center.
    await page.goto('/help');

    // The help center heading should be visible.
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // The owner searches for information about finding customers
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Finding Customers');

    // The results should update, showing the marketing article
    await expect(page.getByText('Send emails to customers')).toBeVisible();

    // They click on it to read the article
    await page.getByText('Send emails to customers').click();

    // And verify the article loaded correctly
    await expect(page.getByRole('heading', { name: 'Finding Customers' })).toBeVisible();
    await expect(page.getByText('Sending Emails')).toBeVisible();
  });

  test('changelog page loads', async ({ page }) => {
    // The owner wants to see what is new in the platform
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();
  });

  test('api docs page loads with warning for advanced users', async ({ page }) => {
    // A technical user or owner trying to integrate something checks the API docs
    await page.goto('/api-docs');

    // It has a warning that it's for advanced users
    await expect(page.getByText('Advanced: This section is for developers directly integrating with our APIs.')).toBeVisible();
  });

});
