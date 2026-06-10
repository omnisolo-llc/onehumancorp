import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('API Docs Page', () => {
  test('navigates to API docs from Help Center', async ({ page }) => {
    await adminPage({ page }, async ({ page }) => {
      // 1. Visit Help Center
      await page.goto('/help');
      await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

      // 2. Find and click API Documentation link
      const apiDocsLink = page.locator('a[href="/api-docs"]', { hasText: 'API Documentation' });
      await expect(apiDocsLink).toBeVisible();
      await apiDocsLink.click();

      // 3. Verify we are on API Docs page
      await expect(page).toHaveURL(/\/api-docs/);
      await expect(page.locator('span', { hasText: 'Advanced:' })).toBeVisible();

      // Wait for swagger UI to render
      await expect(page.locator('.swagger-ui')).toBeVisible();

      // 4. Verify "Back to Help Center" link works
      const backLink = page.locator('a[href="/help"]', { hasText: 'Back to Help Center' });
      await expect(backLink).toBeVisible();
      await backLink.click();

      // 5. Verify back on Help Center
      await expect(page).toHaveURL(/\/help/);
    });
  });
});
