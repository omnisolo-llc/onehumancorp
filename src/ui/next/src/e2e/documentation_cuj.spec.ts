import { test, expect } from '@playwright/test';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();

    // Now Maya navigates to the Help Center
    await page.goto('/help');

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Verify there are articles
    await expect(page.locator('h2:has-text("Articles")')).toBeVisible();

    // Go to getting started
    await page.goto('/help/getting-started-1');
    await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();

    // Check API Docs
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:')).toBeVisible();

    // Check Videos page
    await page.goto('/help/videos');
    await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();
  });
});
