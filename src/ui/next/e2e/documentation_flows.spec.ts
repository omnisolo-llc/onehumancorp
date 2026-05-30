import { test, expect } from '@playwright/test';

test.describe('Documentation Flows', () => {
  test('Help Widget interactions and Videos', async ({ page }) => {
    // Wait for the help page to load
    await page.goto('http://localhost:3000/help');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // Verify the Help floating widget button exists
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton).toBeVisible();

    // Click the widget to open it
    await helpButton.click();

    // Ensure the widget container is visible
    const widgetContainer = page.locator('#help-widget-container').first();
    await expect(widgetContainer).toBeVisible();

    // Change tab to Videos using exact text match
    const videosTab = widgetContainer.locator('button').filter({ hasText: /^Videos$/ });
    await expect(videosTab).toBeVisible();
    await videosTab.click();

    // Click on the first video
    const firstVideo = widgetContainer.locator('div.aspect-\\[9\\/16\\]').first();
    // Wait for videos to load
    await expect(firstVideo).toBeVisible();
    await firstVideo.click();

    // Ensure video player overlay pops up
    const videoOverlayTitle = page.locator('h3', { hasText: 'How to set up your first store easily' });
    await expect(videoOverlayTitle).toBeVisible();
  });
});
