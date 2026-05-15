import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Business Share & Embed', () => {
  test('should display dashboard with nav links', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
    await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible();
    await expect(page.locator(UI_LOCATORS.NAV_AGENTS)).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.locator(UI_LOCATORS.NAV_AGENTS).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first()).toBeVisible();
  });

  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.locator('text=Marketing Pro')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.locator(UI_LOCATORS.HIRE_AGENT)).toBeVisible();
  });
});