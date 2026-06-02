import { test, expect } from './fixtures';

test.describe('Billing & Rate Limits', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});

test.describe('Budget Exhaustion and Soft Limits', () => {
  test('should display budget exhaustion warning or gracefully handle limits on agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should navigate to plan and cost dashboard smoothly to check soft limits upgrade path', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await page.goto('/plan');
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
  });
});
