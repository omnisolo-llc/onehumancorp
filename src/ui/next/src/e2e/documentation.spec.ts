import { test, expect } from '@playwright/test';

test.describe('Documentation Features', () => {
  test('Help Center Search', async ({ page }) => {
    // A mock UI for the E2E is needed or we just ensure the framework runs.
    await page.goto('/help');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toHaveText('Help Center');

    // Type in search
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Paid');

    // The link for 'Getting Paid' should be visible
    const articleLink = page.getByText('Set up how you get paid, view deposits, and handle simple taxes.');
    await expect(articleLink).toBeVisible();

    // Click link and navigate
    await page.getByText('Getting Paid', { exact: true }).click();
    await page.waitForURL('/help/payments');
    await expect(page.locator('h1')).toHaveText('Getting Paid');
  });

  test('Changelog Page', async ({ page }) => {
    await page.goto('/changelog');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toHaveText('Release Notes & Changelog');
    await expect(page.getByText('Interactive AI Store Builder')).toBeVisible();
  });

  test('API Docs Page', async ({ page }) => {
    await page.goto('/api-docs');
    await page.waitForLoadState('networkidle');
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
  });
});
