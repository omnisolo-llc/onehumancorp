import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' }).filter({ visible: true })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').first()).toBeVisible();
    await expect(page.locator('input[type="password"]').first()).toBeVisible();
    await expect(page.locator(SELECTORS.LOGIN_BTN)).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await page.locator('nav a:has-text("Dashboard")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
  });
});