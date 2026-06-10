import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Changelog Page', () => {
  test('navigates to Changelog page from Help Widget', async ({ page }) => {
    await adminPage({ page }, async ({ page }) => {
      // 1. Visit Dashboard (or any page with Help widget)
      await page.goto('/dashboard');

      // 2. Open Help Widget
      const helpButton = page.locator('#help-widget-container button').first();
      await expect(helpButton).toBeVisible();
      await helpButton.click();

      // 3. Open What's New tab
      const whatsNewTab = page.locator('button', { hasText: "What's New" });
      await expect(whatsNewTab).toBeVisible();
      await whatsNewTab.click();

      // 4. Click Read full release notes
      const releaseNotesLink = page.locator('a[href="/changelog"]');
      await expect(releaseNotesLink).toBeVisible();
      await releaseNotesLink.click();

      // 5. Verify we are on Changelog page
      await expect(page).toHaveURL(/\/changelog/);
      await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();

      // 6. Verify "Back to Help Center" link works
      const backLink = page.locator('a[href="/help"]', { hasText: 'Back to Help Center' });
      await expect(backLink).toBeVisible();
      await backLink.click();

      // 7. Verify back on Help Center
      await expect(page).toHaveURL(/\/help/);
    });
  });
});
