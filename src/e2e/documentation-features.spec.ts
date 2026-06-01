import { test, expect } from '@playwright/test';

test.describe('Documentation Features Loop', () => {
  test('User can navigate help center, search, and view articles', async ({ page }) => {
    // Navigate to the Help Center page
    await page.goto('/help');

    // Verify header and search bar
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    await expect(page.locator('input[placeholder="Search for help articles..."]')).toBeVisible();

    // Verify initial article links
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();

    // Search for an article
    await page.locator('input[placeholder="Search for help articles..."]').fill('payments');

    // Verify search results
    await expect(page.locator('h2', { hasText: 'Getting Paid' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).not.toBeVisible();

    // Click on the article link
    await page.click('h2:has-text("Getting Paid")');

    // Verify the article page
    await expect(page.locator('h1', { hasText: 'Getting Paid' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Connecting Your Bank Account' })).toBeVisible();
  });

  test('User can view changelog and api-docs pages', async ({ page }) => {
    // Navigate to the Changelog page
    await page.goto('/changelog');

    // Verify Changelog content
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();

    // Navigate to the API Docs page
    await page.goto('/api-docs');

    // Verify API Docs content
    await expect(page.locator('p', { hasText: 'This section is for developers directly integrating with our APIs. Not required for normal use.' })).toBeVisible();
  });
});
