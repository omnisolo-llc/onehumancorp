import { test, expect } from '@playwright/test';

test.describe('Help Features', () => {
  test('User can search and view articles in Help Center', async ({ page }) => {
    // Navigate directly to help page for the test
    await page.goto('/help');

    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();

    // Search for articles
    await page.fill('input[placeholder="Search for help articles..."]', 'stock');
    await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeHidden();

    // Clear search and click on Getting Started article
    await page.fill('input[placeholder="Search for help articles..."]', '');
    await page.click('text=Getting Started');
    await expect(page.locator('h1', { hasText: 'Getting Started' })).toBeVisible();
    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
  });

  test('User can view Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();
    await expect(page.locator('text=Interactive AI Store Builder:')).toBeVisible();
  });

  test('User can view API Documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced: This section is for developers directly integrating with our APIs.')).toBeVisible();
    // Swagger UI should load (might take a moment to mount the react component)
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });
  });

});
