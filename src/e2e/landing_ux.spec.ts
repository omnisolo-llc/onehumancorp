import { test, expect } from '@playwright/test';

test.describe('Landing Screen Visual Audit', () => {
  test('should display canonical design elements and actions', async ({ page }) => {
    await page.goto('/');

    const startBusinessBtn = page.locator('button:has-text("Start Business Setup")');
    await expect(startBusinessBtn).toBeVisible();

    const continueDashBtn = page.locator('button:has-text("Or continue to Cloud Dashboard")');
    await expect(continueDashBtn).toBeVisible();

    const macDownloadBtn = page.locator('button:has-text("Download for Mac")');
    await expect(macDownloadBtn).toBeVisible();
  });
});
