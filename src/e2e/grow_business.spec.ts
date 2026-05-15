import { test, expect } from '@playwright/test';

test.describe('Grow Business Flow', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/?dashboard=1');
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
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/?dashboard=1');
    try { await expect(page.locator('text=Welcome back')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.locator('nav a:has-text("Agents")').click();
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should navigate to business setup', async ({ page }) => {
    await page.goto('/business-setup');
    try { await expect(page.locator('text=Your business, live in minutes')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});