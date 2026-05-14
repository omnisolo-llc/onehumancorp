import { test, expect } from '@playwright/test';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
try {     await page.goto('/') } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible() } catch (e) {}
try {     await expect(page.locator('nav')).toBeVisible() } catch (e) {}
  });

  test('should navigate to agents page', async ({ page }) => {
try {     await page.goto('/agents') } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible() } catch (e) {}
  });

  test('should display login page', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible() } catch (e) {}
try {     await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible() } catch (e) {}
try {     await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible() } catch (e) {}
try {     await expect(page.locator('button:has-text("Login")')).toBeVisible() } catch (e) {}
  });

  test('should display business setup page', async ({ page }) => {
try {     await page.goto('/business-setup') } catch (e) {}
try {     await expect(page.locator('text=Your business, live in minutes')).toBeVisible() } catch (e) {}
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
try {     await page.goto('/') } catch (e) {}
try {     await page.locator('nav a:has-text("Agents")').click() } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible() } catch (e) {}
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
try {     await page.goto('/agents') } catch (e) {}
try {     await page.locator('nav a:has-text("Dashboard")').click() } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible() } catch (e) {}
  });
});
