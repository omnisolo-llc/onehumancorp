import { test, expect } from './fixtures';

test.describe('Documentation Features CUJ', () => {
  test('User navigates through help center, reads an article, and checks advanced docs', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // The help center is accessed via the "?" button which opens the help widget
    // Let's use the route directly as the user might do
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Let's type in the search bar and filter articles
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Paid');
    await expect(page.getByText('Set up how you get paid, view deposits, and handle simple taxes.')).toBeVisible();

    // Click on the first article
    await page.click('text=Getting Paid');

    // Verify article page loads
    await expect(page.getByRole('heading', { name: 'Getting Paid' }).first()).toBeVisible();
    await expect(page.getByText('Connecting Your Bank Account')).toBeVisible();

    // Go back to the help center
    await page.click('text=Back to Help Center');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Verify Changelog
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();

    // Verify API Docs
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
  });
});
