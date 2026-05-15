import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Email Marketing Flow', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should navigate to login page', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
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