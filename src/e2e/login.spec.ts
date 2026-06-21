import { test, expect } from './fixtures';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Log In', exact: true }).or(page.getByRole('heading', { name: 'Login', exact: true }))).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
  });

  test('should display login button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('button', { name: 'Log In' })).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
  });

  test('should display nav', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation').first()).toBeVisible({ timeout: 15000 });
  });

  test('should show business snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Business Analytics').first()).toBeVisible({ timeout: 15000 });
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Expert Center' })).toBeVisible({ timeout: 15000 });
  });

  test('should display business setup', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=10-Minute Setup Wizard')).toBeVisible({ timeout: 15000 });
  });
});
