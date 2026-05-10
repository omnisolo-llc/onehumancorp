import { test, expect } from '@playwright/test';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('Login screen shows plain language App Settings button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();
    await expect(page.locator('button:has-text("⚙ App Settings")')).toBeVisible();
  });

  test('Login screen shows plain language brand name', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text="One Human Corp"').first()).toBeVisible();
  });

  test('Integrations screen uses plain language for external tools', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Apps")');

    await expect(page.locator('text=Connect Custom Software')).toBeVisible();
  });

  test('API Docs screen uses Custom Integration label', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Apps")');

    await expect(page.locator('text=Custom Integration')).toBeVisible();
  });

  test('API Docs screen replaces GET /v1/products with Read Product List', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Apps")');

    await expect(page.locator('text=Product Data Access')).toBeVisible();
    await expect(page.locator('text=Read Product List')).toBeVisible();
  });
});
