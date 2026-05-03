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
  });

  test('Login screen toggles sign up state', async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await expect(page.locator('button:has-text("Sign Up")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Already have an account? Sign In")')).toBeVisible();
  });

  test('Login screen touch targets are accessible', async ({ page }) => {
    await page.goto('/login');
    const button = page.locator('button:has-text("Sign In")');
    const box = await button.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });

  test('Login screen fits 375px mobile breakpoint', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/login');
    const card = page.locator('text=Sign in to manage your business').locator('..').locator('..');
    const box = await card.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeLessThanOrEqual(375);
    }
  });

  test('Login screen settings button opens settings', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("⚙ App Settings")')).toBeVisible();
    await page.click('button:has-text("⚙ App Settings")');
    await expect(page).toHaveURL(/.*settings.*/);
  });
});
