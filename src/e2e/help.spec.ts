import { test, expect } from './fixtures';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
  });

  test('should allow user to navigate to help center from dashboard', async ({ page }) => {
    await page.goto('/api/ui/dashboard.html');

    // Should see help button in the main navigation or shell
    const helpButton = page.locator('nav').locator('a', { hasText: 'Help' });
    await expect(helpButton).toBeVisible();
    await helpButton.click();
    await expect(page).toHaveURL(/\/api\/ui\/help\.html/);
  });

  test('should provide help resources and allow searching', async ({ page }) => {
    await page.goto('/api/ui/help.html');

    // Help center title should be visible
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Search bar should be functional
    const searchInput = page.locator('input[placeholder*="Search"]');
    await expect(searchInput).toBeVisible();

    await searchInput.fill('payments');

    // There should be search results
    await page.waitForTimeout(500); // Wait for debounce
    const results = page.locator('.help-search-result');
    await expect(results.first()).toBeVisible();
  });

  test('should display contact support option', async ({ page }) => {
    await page.goto('/api/ui/help.html');

    // Should see contact options
    await expect(page.locator('text=Contact Support').or(page.locator('text=Ask AI Agent'))).toBeVisible();
  });
});
