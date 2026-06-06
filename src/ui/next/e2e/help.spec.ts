import { test, expect } from '@playwright/test';

test.describe('Help Center Documentation Flows', () => {
  test('Navigation and Search in Help Center', async ({ page }) => {
    // Navigate to the Help Center
    await page.goto('/help');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // Verify search input is present
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await expect(searchInput).toBeVisible();

    // Verify presence of articles and videos
    await expect(page.getByText('Articles').first()).toBeVisible();
    await expect(page.getByText('Video Tutorials').first()).toBeVisible();

    // Click into an article
    const articleLink = page.getByText('Getting Started', { exact: true });
    await expect(articleLink).toBeVisible();
    await articleLink.click();

    // Verify we navigated to the article page
    await expect(page.locator('h1:has-text("Getting Started with Your Store")')).toBeVisible();

    // Go back to the Help Center
    const backLink = page.getByText('Back to Help Center');
    await expect(backLink).toBeVisible();
    await backLink.click();

    // Test the search functionality
    await searchInput.fill('Payments');
    await expect(page.getByText('Getting Paid')).toBeVisible();
    await expect(page.getByText('My Store')).not.toBeVisible();
  });
});
