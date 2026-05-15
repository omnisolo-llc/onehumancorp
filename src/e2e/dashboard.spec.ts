import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Dashboard Core', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
  });

  test('should show dashboard header', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator('h1').filter({ visible: true }).first()).toBeVisible();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });

  test('should show agents working message', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible();
  });
});

test.describe('Login Page', () => {
  test('should display login page', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first()).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});

test.describe('Business Setup', () => {
  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});