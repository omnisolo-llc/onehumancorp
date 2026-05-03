import { test, expect } from '@playwright/test';

test.describe('Grandmother Test - Plain Language Check', () => {
  test('Login screen uses plain language', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();
    await expect(page.locator('button:has-text("Continue with Google/Apple")')).toBeVisible();
    await expect(page.locator('button:has-text("⚙ App Settings")')).toBeVisible();
  });

  test('Dashboard uses plain language labels', async ({ page }) => {
    // Login first to get to the dashboard
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    // Wait for Dashboard
    await page.waitForURL('**/*');

    await expect(page.locator('text=Orders to Ship')).toBeVisible();
    await expect(page.locator('text=Active Helpers')).toBeVisible();
    await expect(page.locator('text=Active Help')).toBeVisible();
    await expect(page.locator('text=Helper Actions Today')).toBeVisible();
  });
});
