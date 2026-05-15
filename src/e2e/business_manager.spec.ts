import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator(UI_LOCATORS.LOGIN_BUTTON)).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await page.locator(UI_LOCATORS.NAV_AGENTS).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await page.locator('nav a:has-text("Dashboard")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});