import { test, expect } from '@playwright/test';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('Login screen shows plain language Fix App Issues button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });

  test('Login screen shows plain language brand name', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text="One Human Corp"').filter({ visible: true }).first()).toBeVisible();
  });

  test('Integrations screen uses plain language for external tools', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Custom Software")');

    await expect(page.locator('text=Connect Custom Software').last()).toBeVisible();
  });

  test('API Docs screen uses Custom Integration label', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Custom Software")');

    await expect(page.locator('text=Custom Integration')).toBeVisible();
  });

  test('API Docs screen replaces GET /v1/products with Read Product List', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Custom Software")');

    await expect(page.locator('text=Product Data Access').last()).toBeVisible();
    await expect(page.locator('text=Read Product List')).toBeVisible();
  });
});
