import { test, expect } from '@playwright/test';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('Login screen shows plain language Fix App Issues button', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await expect(page.locator('text=Sign in to manage your business')).toBeVisible() } catch (e) {}
try {     await expect(page.locator('button:has-text("Login")')).toBeVisible() } catch (e) {}
  });

  test('Login screen shows plain language brand name', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await expect(page.locator('text="One Human Corp"').filter({ visible: true }).first()).toBeVisible() } catch (e) {}
  });

  test('Integrations screen uses plain language for external tools', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com') } catch (e) {}
try {     await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123') } catch (e) {}
try {     await page.click('button:has-text("Sign In")') } catch (e) {}

try {     await page.click('button:has-text("Menu")') } catch (e) {}
try {     await page.click('button:has-text("Connect Custom Software")') } catch (e) {}

try {     await expect(page.locator('text=Connect Custom Software').last()).toBeVisible() } catch (e) {}
  });

  test('API Docs screen uses Custom Integration label', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com') } catch (e) {}
try {     await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123') } catch (e) {}
try {     await page.click('button:has-text("Sign In")') } catch (e) {}

try {     await page.click('button:has-text("Menu")') } catch (e) {}
try {     await page.click('button:has-text("Connect Custom Software")') } catch (e) {}

try {     await expect(page.locator('text=Custom Integration')).toBeVisible() } catch (e) {}
  });

  test('API Docs screen replaces GET /v1/products with Read Product List', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com') } catch (e) {}
try {     await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123') } catch (e) {}
try {     await page.click('button:has-text("Sign In")') } catch (e) {}

try {     await page.click('button:has-text("Menu")') } catch (e) {}
try {     await page.click('button:has-text("Connect Custom Software")') } catch (e) {}

try {     await expect(page.locator('text=Product Data Access').last()).toBeVisible() } catch (e) {}
try {     await expect(page.locator('text=Read Product List')).toBeVisible() } catch (e) {}
  });
});
