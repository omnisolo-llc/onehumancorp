import { currentAppSmoke } from './current_app_smoke';
import { test, expect } from './fixtures';

currentAppSmoke('test_billing_limits');

test.describe('Billing & Rate Limits', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'AI Agents' }).click();
    await expect(page).toHaveURL(/.*\/agents/);
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
  });
});
