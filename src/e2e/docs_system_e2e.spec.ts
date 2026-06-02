import { test, expect } from '@playwright/test';

test.describe('Documentation System E2E', () => {
  test('should open help center, view tutorials, search articles, and render Swagger API Docs', async ({ page }) => {
    // Navigate to the main application
    await page.goto('/');

    // Wait for the app to load
    await expect(page.locator('body')).toBeVisible();

    // The Help center button `?` should be visible
    const helpBtn = page.locator('#global-help-btn');
    await expect(helpBtn).toBeVisible();

    // Click the Help center button to open the help screen
    await helpBtn.click();

    // Wait for the Help Center screen to be visible
    const helpScreen = page.locator('#help-screen');
    await expect(helpScreen).toBeVisible();

    // Verify getting started and search input are visible
    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
    const searchInput = page.locator('#help-search');
    await expect(searchInput).toBeVisible();

    // Search for a topic
    await searchInput.fill('storefront');
    await searchInput.press('Enter');

    // Check if Video Tutorials section is present
    await expect(page.locator('h2:has-text("Video Tutorials")')).toBeVisible();

    // Verify API documentation page renders Swagger UI
    await page.goto('/api-docs');
    await expect(page.locator('#api-docs-screen')).toBeVisible();

    // Wait for Swagger UI to mount inside the custom styled div
    const swaggerUi = page.locator('#swagger-ui');
    await expect(swaggerUi).toBeVisible();

    // Wait for the swagger content to be visible
    await expect(page.locator('.swagger-ui')).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible();
  });
});
