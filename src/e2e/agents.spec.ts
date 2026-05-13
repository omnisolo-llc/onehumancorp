import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test.describe('Agent Management', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should display hire button', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });

  test('should show marketing agent', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.locator('text=Marketing Pro')).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await expect(page.getByRole('heading', { name: 'Login' }).filter({ visible: true })).toBeVisible();
  });
});