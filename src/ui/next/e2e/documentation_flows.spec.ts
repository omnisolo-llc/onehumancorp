import { test, expect } from '@playwright/test';

test.describe('Documentation Flows', () => {
  test('Help Widget interactions and Videos', async ({ page }) => {
    // Wait for the help page to load
    await page.goto('/help');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    await expect(page.getByPlaceholder('Search for help articles and videos...')).toBeVisible();
    await expect(page.getByText('Articles').or(page.getByText('Video Tutorials')).first()).toBeVisible();
  });
});
