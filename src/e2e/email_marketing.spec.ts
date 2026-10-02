import { test, expect } from './fixtures';

test.describe('Email Marketing Flow', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should navigate to login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: /Sign in|Login/i }).first()).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' }).first()).toBeVisible();
    await page.getByRole('link', { name: 'AI Departments' }).first().click();
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back').first()).toBeVisible();
  });
});
