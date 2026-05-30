import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
  test('navigates to help center and views an article', async ({ page }) => {
    // Navigate to the Help Center
    await page.goto('/help');

    // Verify the Help Center header
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Verify getting started article link exists
    const gettingStartedLink = page.locator('text=Getting Started');
    await expect(gettingStartedLink).toBeVisible();

    // Click on the article
    await gettingStartedLink.click();

    // Verify it navigated to the getting started page
    await expect(page).toHaveURL(/\/help\/getting-started/);

    // Verify the article title
    await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();

    // Verify some content exists using a more specific locator to avoid strict mode violations
    await expect(page.locator('h2', { hasText: 'Step 1: Tell us about your business' })).toBeVisible();
  });
});
