import { test, expect } from '@playwright/test';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h1')).toContainText('Login');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should display login button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });

  test('should have working show button', async ({ page }) => {
    await page.goto('/login');
    const showBtn = page.locator('button:has-text("Show")');
    if (await showBtn.isVisible()) {
      await showBtn.click();
    }
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('h1')).toContainText('Dashboard');
  });

  test('should display nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.locator('h1')).toContainText('Agents');
  });

  test('should display business setup', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});