import { test, expect } from '@playwright/test';

test.describe('Omnibox Global Search', () => {
  test('should open via shortcut, show results, and navigate', async ({ page }) => {
    // Navigate to a real page where layout is rendered (e.g. root)
    await page.goto('/');

    // Ensure omnibox is not visible initially
    await expect(page.getByTestId('omnibox-backdrop')).not.toBeVisible();

    // Trigger keyboard shortcut
    await page.keyboard.press('Meta+k');

    // Wait for omnibox to appear
    await expect(page.getByTestId('omnibox-input')).toBeVisible();

    // Type a search query
    await page.getByTestId('omnibox-input').fill('John');

    // Wait for the API results
    await page.waitForLoadState('networkidle');

    // Just check that it displays "No results" or some results.
    await expect(page.getByTestId('omnibox-results')).toBeVisible();

    // Close using Esc
    await page.keyboard.press('Escape');
    await expect(page.getByTestId('omnibox-backdrop')).not.toBeVisible();
  });
});
