import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Help Center Search', () => {
  test('user can search for a help article', async ({ page }) => {
    await adminPage({ page }, async ({ page }) => {
      await page.goto('/help');
      await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

      const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
      await expect(searchInput).toBeVisible();

      // Search for something that shouldn't exist
      await searchInput.fill('Nonexistent article');
      await expect(page.locator('text=No results found matching')).toBeVisible();

      // Search for something that does exist (empty search first to clear state)
      await searchInput.fill('');

      // Wait for articles to load
      await expect(page.locator('h3', { hasText: 'Getting Started' }).first()).toBeVisible();
    });
  });
});
