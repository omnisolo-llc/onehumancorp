import { test, expect } from '@playwright/test';

test.describe('Help Center Flow', () => {
  test('navigates to help center, searches, and views an article', async ({ page }) => {
    // Navigate directly to the help center page
    await page.goto(`http://localhost:3000/help`);

    // Verify we are on the Help Center page
    await expect(page.locator('h1')).toContainText('Help Center');

    // Search for an article
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Started');

    // Verify the article appears in the search results
    await expect(page.locator('text=Getting Started').first()).toBeVisible();

    // Click on the article link
    await page.locator('text=Getting Started').first().click();

    // Verify we navigated to the article page and content is loaded
    await expect(page.locator('h1')).toContainText('Getting Started');
  });
});
