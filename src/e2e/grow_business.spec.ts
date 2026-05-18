import { test, expect } from './fixtures';

test.describe('Grow Business Flow', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await expect(page.locator('text=Welcome back')).toBeVisible();
    await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should navigate to business setup', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});