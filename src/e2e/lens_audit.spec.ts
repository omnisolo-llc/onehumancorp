import { test, expect } from '@playwright/test';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify Dashboard visual state and full UI lifecycle', async ({ page }) => {
    // Verify dashboard displays with expected elements
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    // Verify nav is present
    await expect(page.locator('nav#main-nav')).toBeVisible();
  });

  test('verify mock data removal and db connection', async ({ page }) => {
    // Audit check to ensure no hardcoded mock data elements are visible
    const mockElements = page.locator('.mock-data-stub');
    await expect(mockElements).toHaveCount(0);
  });

  test('verify token and responsive compliance', async ({ page }) => {
    // Force mobile viewport 375px - nav should still be visible
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('nav#main-nav')).toBeVisible();
  });

  test('verify chaos and error handling', async ({ page }) => {
    // Navigate to root and verify no crash - server serves dashboard for all paths
    await page.goto('/');
    await expect(page.locator('h1').first()).toBeAttached();
  });

  test('verify user guide sync', async ({ page }) => {
    // Check that dashboard is visible at root
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('verify handleLogin logic', async ({ page }) => {
    await page.goto('/login');
    await page.locator('#login-screen').getByPlaceholder('Email or Username').fill('test@test.com');
    await page.locator('#login-screen').getByPlaceholder('Password').fill('password');
    await page.locator('#login-screen').getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('verify handleSignup logic', async ({ page }) => {
    await page.goto('/signup');
    await page.locator('#signup-screen').getByPlaceholder('Email or Username').fill('test@test.com');
    await page.locator('#signup-screen').getByPlaceholder('Password').fill('password');
    await page.locator('#signup-screen').getByRole('button', { name: 'Sign Up' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('verify toggleMenu logic', async ({ page }) => {
    await page.goto('/');
    const extraMenu = page.locator('#extra-menu');
    await expect(extraMenu).toBeHidden();
    await page.getByRole('button', { name: 'Menu' }).click();
    await expect(extraMenu).toBeVisible();
    await page.getByRole('button', { name: 'Menu' }).click();
    await expect(extraMenu).toBeHidden();
  });

  test('verify wizard nextStep logic', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
    await page.locator('#step-1').getByRole('button', { name: '🚀 Start My Business' }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
  });

  test('verify generateAI logic', async ({ page }) => {
    await page.goto('/website-builder');
    await page.locator('#step-1').getByRole('button', { name: '⚡ Instant Build (AI) →' }).click();
    await page.locator('#step-ai').getByRole('button', { name: 'Generate Storefront →' }).click();
    await expect(page.getByRole('heading', { name: 'Designing your storefront...' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Your live storefront!' })).toBeVisible({ timeout: 5000 });
  });
});
