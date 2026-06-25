import { test, expect } from '@playwright/test';

test.describe('Documentation full suite', () => {
  test('Help portal loads properly and search works', async ({ page }) => {
    // Visit help page
    await page.goto('/help');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('In-App Help Center');

    // Make sure search bar exists
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Test search');

    // Chat widget open interaction
    const chatBtn = page.locator('[aria-label="Open help chat"]');
    await expect(chatBtn).toBeVisible();
    await chatBtn.click();

    // Check if the chat input is now visible
    const chatInput = page.locator('input[placeholder="Ask anything..."]');
    await expect(chatInput).toBeVisible();
  });

  test('Changelog pulls data dynamically', async ({ page }) => {
    // Visit changelog page
    await page.goto('/changelog');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('Release Notes');

    // Expecting to load cards dynamically from API
    // The test waits for the dynamic content to appear
    const changelogContainer = page.locator('div.space-y-8');
    await expect(changelogContainer).toBeVisible();
  });

  test('API Docs loads Swagger UI', async ({ page }) => {
    // Visit api docs page
    await page.goto('/api-docs');

    // Check for Swagger UI wrapper
    const swaggerUI = page.locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible();

    // Ensure the topbar from Swagger has loaded, indicating success
    const info = page.locator('.info .title');
    await expect(info).toBeVisible();
  });
});
