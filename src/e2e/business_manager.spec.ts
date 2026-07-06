import { test, expect } from './fixtures';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Log In' })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/setup.html');
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.locator('#setup-screen')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    const link = page.getByRole('link', { name: 'AI Departments', exact: true });
    await expect(link).toBeVisible();
    await link.click();
    await page.waitForURL('**/agents**');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('header a[href="/dashboard"]').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
