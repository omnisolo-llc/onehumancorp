import { test, expect } from '@playwright/test';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('h1')).toContainText('Dashboard');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1')).toContainText('Agents');
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h1')).toContainText('Login');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
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
    await expect(page.locator('h1')).toContainText('Agents');
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('nav a:has-text("Dashboard")').click();
    await expect(page.locator('h1')).toContainText('Dashboard');
  });
});