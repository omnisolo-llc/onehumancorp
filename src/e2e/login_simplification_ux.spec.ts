import { test, expect } from '@playwright/test';

test.describe('Login Screen Simplification Audit', () => {
  test('should display only One Human Corp heading', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'One Human Corp' })).toBeVisible();
    await expect(page.locator('h1', { hasText: 'Login' })).not.toBeVisible();
  });

  test('should display only one login action', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('button', { name: 'Sign In' }).filter({ visible: true })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Fix App Issues' })).not.toBeVisible();
    await expect(page.locator('button', { hasText: 'Login' })).not.toBeVisible();
  });

  test('should navigate to dashboard upon login', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: 'Sign In' }).filter({ visible: true }).first().click();
    await page.waitForURL('**/*');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should navigate to setup screen upon starting setup', async ({ page }) => {
    await page.goto('/login');
    await page.locator('button:has-text("🚀 Start Business Setup")').click();
    await expect(page.locator('h1', { hasText: 'Business Setup' })).toBeVisible();
  });

  test('should navigate to signup screen', async ({ page }) => {
    await page.goto('/login');
    await page.locator('button:has-text("Don\\'t have an account? Sign Up")').click();
    await expect(page.getByRole('heading', { name: 'Create an account' })).toBeVisible();
  });
});
