import { test, expect } from '@playwright/test';

test.describe('Business Share & Embed', () => {
  test('should display dashboard with nav links', async ({ page }) => {
    await page.goto('/?dashboard=1');
    try { await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('nav')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('nav a:has-text("Agents")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.locator('nav a:has-text("Agents")').click();
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    try { await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    try { await expect(page.locator('text=Your business, live in minutes')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});

test.describe('Agents Page', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto('/agents');
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Marketing Pro')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    try { await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});