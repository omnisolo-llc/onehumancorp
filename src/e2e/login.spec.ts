import { test, expect } from './fixtures';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ anonymousPage }) => {
    await anonymousPage.goto('/login');
    await expect(anonymousPage.getByRole('heading', { name: /Sign in|Login/i }).first()).toBeVisible();
    await expect(anonymousPage.getByLabel(/Email or username/i)).toBeVisible();
    await expect(anonymousPage.getByLabel(/Password/i)).toBeVisible();
  });

  test('should display login button', async ({ anonymousPage }) => {
    await anonymousPage.goto('/login');
    await expect(anonymousPage.getByRole('button', { name: /Log in/i })).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should display nav', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible({ timeout: 15000 });
  });

  test('should show business snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible({ timeout: 15000 });
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1.app-title, h1', { hasText: /Agents|AI Departments/i }).first()).toBeVisible({ timeout: 15000 });
  });

  test('should display business setup', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: /Setup Assistant|10-Minute Setup Wizard/i }).or(page.locator('h1')).first()).toBeVisible({ timeout: 15000 });
  });
});
