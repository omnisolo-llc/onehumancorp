import { test, expect } from './fixtures';

test.describe('Login Screen Visual Audit', () => {
  test('should display login page', async ({ anonymousPage }) => {
    await anonymousPage.goto('/login');
    await expect(anonymousPage.getByRole('heading', { name: /Sign in|Login/i }).first()).toBeVisible();
    await expect(anonymousPage.getByLabel(/Email or username/i)).toBeVisible();
    await expect(anonymousPage.getByLabel(/Password/i)).toBeVisible();
  });

  test('should navigate to dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should navigate to onboarding', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByRole('heading').or(page.getByText(/Tell us about your business|onboarding/i)).first()).toBeVisible();
  });

  test('should display dashboard directly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' }).first()).toBeVisible();
    await page.getByRole('link', { name: 'AI Departments' }).first().click();
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back').first()).toBeVisible();
  });
});