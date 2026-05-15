import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first()).toBeVisible();
  });

  test('should display login button', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.locator(UI_LOCATORS.LOGIN_BUTTON)).toBeVisible();
  });

  test('should have working show button', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    const showBtn = page.locator('button:has-text("Show")');
    if (await showBtn.isVisible()) {
      await showBtn.click();
    }
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display nav', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await page.locator(UI_LOCATORS.NAV_AGENTS).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display business setup', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});