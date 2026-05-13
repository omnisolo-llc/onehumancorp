import { test, expect } from '@playwright/test';

test.describe('Landing Screen Visual Audit', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('h1')).toContainText('Dashboard');
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h1')).toContainText('Login');
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1')).toContainText('Agents');
  });
});