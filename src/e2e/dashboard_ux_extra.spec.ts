import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Friction Fix Verification', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Dashboard');
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.locator('h1')).toContainText('Agents');
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h1')).toContainText('Login');
  });
});