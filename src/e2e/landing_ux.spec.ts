import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test.describe('Landing Screen Visual Audit', () => {
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

  test('should display agents page', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });
});