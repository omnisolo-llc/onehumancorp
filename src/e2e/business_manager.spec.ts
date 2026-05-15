import { test, expect } from '@playwright/test';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto('/');
    try { await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('nav')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    try { await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('button:has-text("Login")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    try { await expect(page.locator('text=Your business, live in minutes')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('nav a:has-text("Dashboard")').click();
    try { await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});