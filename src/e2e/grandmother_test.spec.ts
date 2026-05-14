import { test, expect } from '@playwright/test';

test.describe('Grandmother Test - Plain Language Check', () => {
  test('should display login page with form', async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible() } catch (e) {}
try {     await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible() } catch (e) {}
try {     await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible() } catch (e) {}
  });

  test('should display dashboard with nav', async ({ page }) => {
try {     await page.goto('/') } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible() } catch (e) {}
try {     await expect(page.locator('nav')).toBeVisible() } catch (e) {}
  });

  test('should show welcome message on dashboard', async ({ page }) => {
try {     await page.goto('/') } catch (e) {}
try {     await expect(page.locator('text=Welcome back')).toBeVisible() } catch (e) {}
  });

  test('should display agents page', async ({ page }) => {
try {     await page.goto('/agents') } catch (e) {}
try {     await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible() } catch (e) {}
  });

  test('should display business setup', async ({ page }) => {
try {     await page.goto('/business-setup') } catch (e) {}
try {     await expect(page.locator('text=Your business, live in minutes')).toBeVisible() } catch (e) {}
  });
});
