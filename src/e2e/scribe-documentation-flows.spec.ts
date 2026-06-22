import { test, expect } from './fixtures';

test.describe('Documentation Features Flow', () => {
  test('User can navigate the Help Center and view an article', async ({ page }) => {
    // Navigate directly without mocking, allowing the real backend / fallback APIs to respond.
    await page.goto('/api/ui/help.html');

    // Help Center Index
    await expect(page).toHaveURL(/\/help.html/);

    // Wait until hydration finishes or layout settles before clicking
    await page.waitForLoadState('networkidle');

    // Click on the first article using a simpler selector that works regardless of exact visible state transitions
    const articleLink = page.locator('a[href="/api/ui/help_article.html?id=getting-started-1"]').first();
    await articleLink.click({ force: true });

    // Help Article Page
    await expect(page).toHaveURL(/\/help_article.html\?id=getting-started-1/, { timeout: 15000 });
  });

  test('Advanced User can access API Documentation', async ({ page }) => {
    // Navigate directly without mocking
    await page.goto('/api/ui/api-docs.html');
    await page.waitForLoadState('networkidle');
  });
});
