import { test, expect } from '@playwright/test';

test.describe('Documentation Feature E2E Tests', () => {
  test('Verify help center and navigation', async ({ page }) => {
    // Check Help Center
    await page.goto('http://localhost:3000/help');
    await page.waitForTimeout(1000);
    await expect(page.locator('h1')).toContainText('Help Center');

    // Check search functionality
    await page.fill('input[placeholder="Search for help articles..."]', 'Getting Paid');
    await expect(page.locator('text=Getting Paid')).toBeVisible();
    await expect(page.locator('text=My Store')).not.toBeVisible();

    // Check specific article
    await page.goto('http://localhost:3000/help/getting-started');
    await page.waitForTimeout(1000);
    await expect(page.locator('h1')).toContainText('Getting Started with Your Store');

    // Verify back navigation
    await page.click('text=Back to Help Center');
    await expect(page.url()).toContain('/help');
  });

  test('Verify API Docs loads Swagger UI', async ({ page }) => {
    await page.goto('http://localhost:3000/api-docs');
    await page.waitForTimeout(1000);

    // Check the advanced warning is present
    await expect(page.locator('text=Advanced:')).toBeVisible();

    // Check if Swagger UI rendered
    await expect(page.locator('.swagger-ui')).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible();
  });

  test('Verify Changelog rendering', async ({ page }) => {
    await page.goto('http://localhost:3000/changelog');
    await page.waitForTimeout(1000);

    await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
    await expect(page.locator('h2')).toContainText('Version 1.0 (Latest)');
    await expect(page.locator('text=Interactive AI Store Builder:')).toBeVisible();
  });
});
