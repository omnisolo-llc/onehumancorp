import { test, expect } from './fixtures';

test.describe('Lens Audit Visual Checks', () => {
  test('should display Expert Center heading on agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Expert Center' })).toBeVisible();
  });

  test('should navigate to dashboard and show welcome message', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });

  test('should display dashboard correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
  });

  test('should navigate to website builder', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=Setup Assistant')).toBeVisible({ timeout: 15000 });
  });

  test('should display login fields', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Log In' })).toBeVisible();
  });
});
