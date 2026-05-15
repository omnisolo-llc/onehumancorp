import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Security Settings', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate between pages via nav', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
    await page.locator(UI_LOCATORS.NAV_AGENTS).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should have nav links to all main sections', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible();
    await expect(page.locator(UI_LOCATORS.NAV_AGENTS)).toBeVisible();
    await expect(page.locator('nav a:has-text("Setup")')).toBeVisible();
  });
});