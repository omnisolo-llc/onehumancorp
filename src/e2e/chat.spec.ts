import { test, expect } from './fixtures';

test.describe('Chat Page', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should display login page', async ({ anonymousPage }) => {
    await anonymousPage.goto('/login');
    await expect(anonymousPage.getByRole('heading', { name: /Sign in|Login/i }).first()).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Setup Assistant' }).or(page.locator('h1')).first()).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
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