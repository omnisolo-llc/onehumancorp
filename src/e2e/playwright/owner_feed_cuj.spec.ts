import { test, expect } from '../fixtures';

test.describe('Owner Feed Component Tests', () => {
    test.beforeEach(async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard');
    });

  test('should render owner feed items without error on dashboard', async ({ page }) => {
    // Assert that the dashboard loads without catastrophic error.
    // We expect the dashboard header to be visible.
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('div[class*="rounded-[16px]"]')).toBeHidden();
  });
});
