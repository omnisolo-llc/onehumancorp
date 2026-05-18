import { test, expect } from './fixtures';

test.describe('Email Marketing Flow', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should navigate to login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});