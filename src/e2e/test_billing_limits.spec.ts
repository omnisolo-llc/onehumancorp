import { test, expect } from './fixtures';

test.describe('Billing & Rate Limits', () => {
  test('should display dashboard', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});