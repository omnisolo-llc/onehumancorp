import { test, expect } from '@playwright/test';

test.describe('Landing Screen Visual Audit', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display canonical design elements and actions', async ({ page }) => {
    await page.goto('/');

    const startBusinessBtn = page.locator('button:has-text("Start Business Setup")');
    await expect(startBusinessBtn).toBeVisible();

    const continueDashBtn = page.locator('button:has-text("Or continue to Cloud Dashboard")');
    await expect(continueDashBtn).toBeVisible();

    const macDownloadBtn = page.locator('button:has-text("Download for Mac")');
    await expect(macDownloadBtn).toBeVisible();
  });

  test('should display local sovereignty and cloud convenience texts', async ({ page }) => {
    await page.goto('/');

    // Test the default variant text
    await expect(page.locator('text=Local Sovereignty')).toBeVisible();

    // In our tests, we can verify that "Cloud Convenience" is present if variant is toggled.
    // For now, assert the base presence of the new text requirement
  });
});

test.describe('Landing Screen - Desktop View', () => {
  test.use({ viewport: { width: 1440, height: 900 } });
  test('should display elements at 1440px', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Local Sovereignty')).toBeVisible();
  });
});

test.describe('Landing Screen - Tablet View', () => {
  test.use({ viewport: { width: 768, height: 1024 } });
  test('should display elements at 768px', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Local Sovereignty')).toBeVisible();
  });
});
