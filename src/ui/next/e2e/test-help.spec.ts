import { test, expect } from '@playwright/test';

test.describe('Help Center & Documentation', () => {
  test('Verify help center navigation and UI', async ({ page }) => {
    // Navigate to Help Center
    await page.goto('http://localhost:3000/help');

    // Check main heading
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // Check if articles are loaded
    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();

    // Click on Getting Started article
    await page.click('h2:has-text("Getting Started")');

    // Wait for navigation and verify article content
    await expect(page.locator('h1:has-text("Getting Started with Your Store")')).toBeVisible();
  });

  test('Verify API Documentation page', async ({ page }) => {
    // Navigate to API Docs
    await page.goto('http://localhost:3000/api-docs');

    // Check if Swagger UI is loaded
    await expect(page.locator('text="OHC Advanced API Reference"')).toBeVisible();
  });

  test('Verify Changelog page', async ({ page }) => {
    // Navigate to Changelog
    await page.goto('http://localhost:3000/changelog');

    // Check main heading
    await expect(page.locator('h1:has-text("Release Notes & Changelog")')).toBeVisible();

    // Check content
    await expect(page.locator('h2:has-text("Version 1.0 (Latest)")')).toBeVisible();
  });

  test('Verify Help Chat Widget', async ({ page }) => {
    // Intercept API call
    await page.route('/api/chat', async route => {
      const json = {
        reply: "I am your AI Help Agent!",
        link: { url: "/help", title: "Read the full article →" }
      };
      await route.fulfill({ json });
    });

    // Go to any page that has the layout (e.g. Help Center)
    await page.goto('http://localhost:3000/help');

    // Click the floating chat button
    await page.click('button:has-text("Ask anything")');

    // Verify chat window opens
    await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();

    // Type a message and send
    await page.fill('input[placeholder="Ask me anything..."]', 'How do I add a product?');
    await page.click('button[type="submit"]');

    // Verify response
    await expect(page.locator('text=I am your AI Help Agent!')).toBeVisible();
    await expect(page.locator('text=Read the full article →')).toBeVisible();
  });
});
