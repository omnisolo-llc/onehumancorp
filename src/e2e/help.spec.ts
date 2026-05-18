import { test, expect } from './fixtures';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.locator('nav a:has-text("Dashboard")');
    await expect(dashLink).toBeVisible();
  });

  test('should show agents link in nav', async ({ page }) => {
    const agentsLink = page.locator('nav a:has-text("Agents")');
    await expect(agentsLink).toBeVisible();
  });

  test('should show setup link in nav', async ({ page }) => {
    const setupLink = page.locator('nav a:has-text("Setup")');
    await expect(setupLink).toBeVisible();
  });

  test('should display welcome message', async ({ page }) => {
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });

  test('should display agents working message', async ({ page }) => {
    await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible();
  });
});

test.describe('Login Page', () => {
  test('should display login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });
});

test.describe('Business Setup Page', () => {
  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.getByRole('heading', { name: 'OneHuman' })).toBeVisible();
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
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});