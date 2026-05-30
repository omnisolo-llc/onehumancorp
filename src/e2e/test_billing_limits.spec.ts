import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

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
    await page.locator('nav a[href="/agents"]').first().click();
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});