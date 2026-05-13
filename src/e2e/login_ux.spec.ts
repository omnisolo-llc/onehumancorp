import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test.describe('Login Screen Visual Audit', () => {
  test('should display login page', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' }).filter({ visible: true })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').first()).toBeVisible();
    await expect(page.locator('input[type="password"]').first()).toBeVisible();
  });

  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});