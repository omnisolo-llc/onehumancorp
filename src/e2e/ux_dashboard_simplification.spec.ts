import { test, expect } from './fixtures';

test.describe('Dashboard UX Simplification (Grandmother Test)', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Daily Briefing' })).toBeVisible();
    await expect(page.locator('text=Welcome back!')).toBeVisible();
  });

  test('should display daily briefing active customers count', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=active customers')).toBeVisible();
  });

  test('should display daily briefing sales information', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Today\'s sales have reached')).toBeVisible();
  });

  test('should display AI task summary in daily briefing', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=The AI has drafted an Instagram post')).toBeVisible();
  });

  test('should display pending orders count', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=new pending orders')).toBeVisible();
  });
});