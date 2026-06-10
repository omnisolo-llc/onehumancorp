import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Dashboard Links and Navigation E2E', () => {

  test('should load dashboard successfully', async ({ page }) => {
    await page.goto('http://127.0.0.1:3000/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Business Analytics' }).first()).toBeVisible();
  });

  test('should not contain the removed "Store Manager Chat" link pointing to /chat', async ({ page }) => {
    await page.goto('http://127.0.0.1:3000/dashboard');
    const chatLink = page.locator('a[href="/chat"]');
    await expect(chatLink).toHaveCount(0);
  });

  test('should successfully navigate to /settings from the dashboard', async ({ page }) => {
    await page.goto('http://127.0.0.1:3000/dashboard');
    const settingsLink = page.locator('a[href="/settings"]').first();
    await settingsLink.click();
    await page.waitForURL('http://127.0.0.1:3000/settings');

    // Validate we are on the settings page
    await expect(page.locator('h1', { hasText: 'Settings' }).first()).toBeVisible();
  });

});
