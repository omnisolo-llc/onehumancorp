import { test, expect } from '@playwright/test';

test.describe('Documentation Features Verification', () => {
  test('Help Center search filters articles', async ({ page }) => {
    await page.goto('http://localhost:3000/help');
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    const searchInput = page.locator('input[placeholder="Search for help articles..."]');
    await searchInput.fill('Getting Started');

    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();
  });

  test('API Docs page loads correctly', async ({ page }) => {
    await page.goto('http://localhost:3000/api-docs');
    await expect(page.locator('text=Advanced: This section is for developers')).toBeVisible();
  });

  test('Dashboard tooltips trigger on hover', async ({ page }) => {
    await page.goto('http://localhost:3000/dashboard');
    const getWidgetBtn = page.locator('button:has-text("Get Widget")').first();
    await expect(getWidgetBtn).toBeVisible();

    // Hover over the element that wraps the button (the WithTooltip container)
    // We hover the button itself, and the parent container should handle the event
    await getWidgetBtn.hover();

    // Wait for the tooltip text to appear
    await expect(page.locator('text=Get an HTML snippet to easily embed your storefront anywhere.')).toBeVisible();
  });
});
