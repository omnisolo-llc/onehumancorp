import { test, expect } from '@playwright/test';

test.describe('Documentation Features Flow', () => {
  test('User can navigate the Help Center and view an article', async ({ page }) => {
    // Navigate directly without mocking, allowing the real backend / fallback APIs to respond.
    await page.goto('/help');

    // Help Center Index
    await expect(page).toHaveURL(/\/help/);

    // Wait until hydration finishes or layout settles before clicking
    await page.waitForLoadState('networkidle');

    // Click on the first article using a simpler selector that works regardless of exact visible state transitions
    const articleLink = page.locator('a[href="/help/getting-started-1"]').first();
    await articleLink.click({ force: true });

    // Help Article Page
    await expect(page).toHaveURL(/\/help\/getting-started-1/, { timeout: 15000 });
  });

  test('User can search the Help Center', async ({ page }) => {
    await page.goto('/help');
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await expect(searchInput).toBeVisible();

    await searchInput.fill('Payments');
    await page.waitForTimeout(500); // wait for typing debounce/search render
    await expect(page.locator('h3:has-text("Accepting Payments")')).toBeVisible();
  });

  test('User can open and use Help Chat', async ({ page }) => {
    await page.goto('/help');
    const chatBtn = page.locator('button[aria-label="Open help chat"]');
    await expect(chatBtn).toBeVisible();
    await chatBtn.click();
    await expect(page.locator('input[placeholder="Ask anything..."]')).toBeVisible();
  });

  test('Advanced User can access API Documentation', async ({ page }) => {
    // Navigate directly without mocking
    await page.goto('/api-docs');
    await page.waitForLoadState('networkidle');
    const swaggerUI = page.locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible({ timeout: 15000 });
  });
});
