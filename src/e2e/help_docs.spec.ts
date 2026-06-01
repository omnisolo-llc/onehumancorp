import { test, expect } from '@playwright/test';

test('Verify Help and Documentation UX flow', async ({ page }) => {
  // Navigate to Help Center
  await page.goto('http://localhost:3000/help');
  await page.waitForTimeout(1000);

  // Verify Search Input exists and can be typed into
  const searchInput = page.locator('input[placeholder="Search for help articles..."]');
  await expect(searchInput).toBeVisible();
  await searchInput.fill('Getting Started');

  // Check for article presence
  const articleHeading = page.locator('h2', { hasText: 'Getting Started' });
  await expect(articleHeading).toBeVisible();

  // Navigate to API Docs
  await page.goto('http://localhost:3000/api-docs');
  await page.waitForTimeout(2000); // Allow Swagger UI to mount

  // Verify Swagger UI renders successfully
  const swaggerContainer = page.locator('.swagger-ui').first();
  await expect(swaggerContainer).toBeVisible();

  // Navigate to Changelog
  await page.goto('http://localhost:3000/changelog');
  await page.waitForTimeout(1000);

  // Verify Changelog loads correctly
  const changelogHeader = page.locator('h1', { hasText: 'Release Notes' });
  await expect(changelogHeader).toBeVisible();
  const firstVersion = page.locator('h2', { hasText: 'Version 1.0 (Latest)' });
  await expect(firstVersion).toBeVisible();
});
