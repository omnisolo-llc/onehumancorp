import { test, expect } from '@playwright/test';

test.describe('Documentation Flows', () => {

  test.beforeEach(async ({ page }) => {
    // Mock the backend API responses required for the help center to load correctly
    await page.route('**/api/help', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { category: "Getting Started", id: "getting-started-1", title: "Getting Started with Your Store", desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.", link: "/help/getting-started-1" },
          { category: "My Store", id: "add-products", title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/add-products" }
        ])
      });
    });

    await page.route('**/api/videos', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{ id: 1, title: "Set up your store", duration: "1:15", video_url: "https://example.com/video.mp4" }])
      });
    });

    await page.route('**/api/tooltips', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          "help-btn-tooltip-appshell": "Need help? Click here to access our Help Center and tutorials."
        })
      });
    });
  });

  test('Help Widget interactions and Videos', async ({ page }) => {
    // Wait for the help page to load
    await page.goto('/help');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    await expect(page.locator('input[placeholder="Search for help articles and videos..."]')).toBeAttached();
    // Verify articles are rendered from the mock
    await expect(page.getByText('Adding Products').first()).toBeVisible();
  });

  test('Tooltips load and display properly', async ({ page }) => {
    // Go to a page with the help widget
    await page.goto('/help');

    // Make sure the help button exists
    const helpBtn = page.getByRole('button', { name: 'Help', exact: true });
    await expect(helpBtn).toBeVisible();

    // Hover over the help button to trigger the tooltip
    await page.locator('#help-btn-tooltip-appshell').dispatchEvent('touchstart');
    await page.waitForTimeout(600); // 500ms for long press

    // Verify the tooltip loads with expected content from our mock
    const tooltipText = page.getByText(/Need help\? Click here/i).last();
    await expect(tooltipText).toBeVisible();
  });
});
