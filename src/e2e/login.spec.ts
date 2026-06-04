import { test, expect } from './fixtures';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.locator('input[type="text"]').first()).toBeVisible();
    await expect(page.locator('input[type="password"]').first()).toBeVisible();
  });

  test('should display login button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });

  test('should have email label', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByText('Email', { exact: true }).first()).toBeVisible();
  });

  test('should have password label', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByText('Password', { exact: true }).first()).toBeVisible();
  });

  test('should display subtitle', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByText('Access your OHC Workspace').first()).toBeVisible();
  });
});
