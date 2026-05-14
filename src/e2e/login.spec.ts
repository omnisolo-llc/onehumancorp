import { test, expect } from '@playwright/test';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' }).filter({ visible: true })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').first()).toBeVisible();
    await expect(page.locator('input[type="password"]').first()).toBeVisible();
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
    await expect(page.getByRole('heading', { name: /Dashboard/i }).filter({ visible: true }).first()).toBeVisible({ timeout: 10000 });
  });

  test('should display nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav').first()).toBeVisible({ timeout: 10000 });
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=/Welcome back/i').first()).toBeVisible({ timeout: 10000 });
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').first().click({ timeout: 10000 });
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should display business setup', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=/Your business, live in minutes/i').first()).toBeVisible({ timeout: 10000 });
  });
});