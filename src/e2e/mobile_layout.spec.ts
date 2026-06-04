import { test, expect } from '@playwright/test';

test.describe('Mobile First Design', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // iPhone SE viewport

  test('app shell is responsive and usable at 375px', async ({ page }) => {
    // Navigate using relative URL
    await page.goto('/dashboard');

    // Wait for the page to load
    await expect(page.locator('.app-title')).toBeVisible();

    // Check that we can see the sidebar navigation icons, but we might be in a row
    const nav = page.locator('.app-nav').first();
    await expect(nav).toBeVisible();

    // The grid should stack cards
    const metricsGrid = page.locator('.app-grid.metrics');
    await expect(metricsGrid).toBeVisible();

    // We can evaluate if flex direction is column on the grid
    const flexDir = await metricsGrid.evaluate((el) => {
      return window.getComputedStyle(el).gridTemplateColumns;
    });

    console.log("Grid template columns on mobile:", flexDir);
  });
});
