import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test.describe('Security Settings', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' }).filter({ visible: true })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate between pages via nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should have nav links to all main sections', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible();
    await expect(page.locator('nav a:has-text("Agents")')).toBeVisible();
    await expect(page.locator('nav a:has-text("Setup")')).toBeVisible();
  });
});