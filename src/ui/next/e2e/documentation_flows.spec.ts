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

  test('Tooltips load and display properly', async ({ page }) => {
    // Go to a page with the help widget
    await page.goto('/help');

    // Make sure the help button exists
    const helpBtn = page.getByRole('button', { name: 'Help', exact: true });
    await expect(helpBtn).toBeVisible();

    // Hover over the help button to trigger the tooltip
    await helpBtn.hover();

    // Verify the tooltip loads with expected content
    // We expect the tooltip to fetch from the API which defaults to "Need help? Click here for guides, videos, and to ask our AI." or the defaultText "Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes."
    // Because the rust backend returns: "Need help? Click here to access our Help Center and tutorials."
    const tooltipText = page.getByText(/Need help\? Click here/i).last();
    await expect(tooltipText).toBeVisible();
  });
});
