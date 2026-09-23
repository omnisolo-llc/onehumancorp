import { test, expect } from './fixtures';

test.describe('Business Share & Embed', () => {
  test('should display dashboard with nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByRole('link', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'AI Departments' }).first().click();
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: /Sign in|Login/i }).first()).toBeVisible();
    await expect(page.getByLabel(/Email or username/i)).toBeVisible();
    await expect(page.getByLabel(/Password/i)).toBeVisible();
  });

  test('should display setup page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=Setup Assistant, h1').first()).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.locator('text=The Promoter')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=My Team')).toBeVisible();
  });
});