import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Agent Management', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display hire button', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.locator(UI_LOCATORS.HIRE_AGENT)).toBeVisible();
  });

  test('should show marketing agent', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.locator('text=Marketing Pro')).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});