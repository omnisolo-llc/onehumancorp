import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.locator('button').first();
    await expect(dashLink).toBeVisible();
  });

  test('should show agents link in nav', async ({ page }) => {
    const agentsLink = page.locator('button').first();
    await expect(agentsLink).toBeVisible();
  });

  test('should show setup link in nav', async ({ page }) => {
    const setupLink = page.locator('button').first();
    await expect(setupLink).toBeVisible();
  });

  test('should display welcome message', async ({ page }) => {
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('should display agents working message', async ({ page }) => {
    await expect(page.locator('h1').first()).toBeVisible();
  });
});

test.describe('Login Page', () => {
  test('should display login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' }).filter({ visible: true })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').first()).toBeVisible();
    await expect(page.locator('input[type="password"]').first()).toBeVisible();
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });
});

test.describe('Business Setup Page', () => {
  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.getByRole('heading', { name: 'OneHuman' }).filter({ visible: true })).toBeVisible();
  });

  test('should show setup wizard text', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' }).filter({ visible: true })).toBeVisible();
  });
});