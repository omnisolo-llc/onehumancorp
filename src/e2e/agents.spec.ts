import { test, expect } from './fixtures';

test.describe('Agent Management', () => {
  test('should display agents page (AI Departments)', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should show The Ambassador department', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=Customer Success')).toBeVisible();
  });

  test('should show The Manager department', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=The Manager')).toBeVisible();
    await expect(page.locator('text=Operations')).toBeVisible();
  });

  test('should show The Closer department', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=The Closer')).toBeVisible();
    await expect(page.locator('text=Sales')).toBeVisible();
  });

  test('should toggle department settings', async ({ page }) => {
    await page.goto('/agents');

    // Settings should be hidden initially
    await expect(page.locator('text=Auto-approve: $0')).not.toBeVisible();

    // Click the Advanced toggle to show settings
    await page.locator('span:has-text("Advanced")').locator('..').locator('button').click();

    await expect(page.locator('text=Auto-approve: $0').first()).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});