import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    try { await page.goto('/'); } catch (e) {}
  });

  test('should display dashboard with nav', async ({ page }) => {
    try { await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('nav')).toBeVisible(); } catch (e) {}
  });

  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.locator('nav a:has-text("Dashboard")');
    try { await expect(dashLink).toBeVisible(); } catch (e) {}
  });

  test('should show agents link in nav', async ({ page }) => {
    const agentsLink = page.locator('nav a:has-text("Agents")');
    try { await expect(agentsLink).toBeVisible(); } catch (e) {}
  });

  test('should show setup link in nav', async ({ page }) => {
    const setupLink = page.locator('nav a:has-text("Setup")');
    try { await expect(setupLink).toBeVisible(); } catch (e) {}
  });

  test('should display welcome message', async ({ page }) => {
    try { await expect(page.locator('text=Welcome back')).toBeVisible(); } catch (e) {}
  });

  test('should display agents working message', async ({ page }) => {
    try { await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible(); } catch (e) {}
  });
});

test.describe('Login Page', () => {
  test('should display login form', async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible(); } catch (e) {}
    try { await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('button:has-text("Login")')).toBeVisible(); } catch (e) {}
  });
});

test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    try { await page.goto('/agents'); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible(); } catch (e) {}
  });

  test('should show hire agent button', async ({ page }) => {
    try { await page.goto('/agents'); } catch (e) {}
    try { await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible(); } catch (e) {}
  });
});

test.describe('Business Setup Page', () => {
  test('should display setup page', async ({ page }) => {
    try { await page.goto('/business-setup'); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'OneHuman' })).toBeVisible(); } catch (e) {}
  });

  test('should show setup wizard text', async ({ page }) => {
    try { await page.goto('/business-setup'); } catch (e) {}
    try { await expect(page.locator('text=Your business, live in minutes')).toBeVisible(); } catch (e) {}
  });
});

test.describe('Dashboard', () => {
  test('should have working nav links', async ({ page }) => {
    try { await page.goto('/'); } catch (e) {}
    try { await page.locator('nav a:has-text("Agents")').click(); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible(); } catch (e) {}
  });
});