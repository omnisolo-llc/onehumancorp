import { test, expect } from './fixtures';

test.describe('Chat Page', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=Setup Assistant')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
    await page.getByRole('link', { name: 'AI Departments' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});

test.describe('Omnichannel Inbox UI', () => {
  test('should display unified inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
    // Use proper locator avoiding strict issues or fake content
  });
});
