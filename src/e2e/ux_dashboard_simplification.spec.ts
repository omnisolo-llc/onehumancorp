import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Simplification (Grandmother Test)', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
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
  test('should navigate via nav links', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
    await page.locator(UI_LOCATORS.NAV_AGENTS).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});