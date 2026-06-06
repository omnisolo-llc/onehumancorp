import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display database-backed operations console', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.locator('text="Operations Map"').first()).toBeVisible();
    await expect(page.locator('text="Action Required"').first()).toBeVisible();
    await expect(page.locator('text="Recent Orders"')).toBeVisible();
    await expect(page.locator('text="Inbox Activity"')).toBeVisible();
  });
});
