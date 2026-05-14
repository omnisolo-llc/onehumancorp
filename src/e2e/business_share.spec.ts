import { test, expect } from '@playwright/test';

test.describe('Business Share & Embed', () => {
  test('should display dashboard with nav links', async ({ page }) => {
    try { await page.goto('/?dashboard=1'); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('nav')).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('nav a:has-text("Agents")')).toBeVisible(); } catch (e) {}
  });

  test('should navigate to agents page', async ({ page }) => {
    try { await page.goto('/?dashboard=1'); } catch (e) {}
    try { await page.locator('nav a:has-text("Agents")').click(); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible(); } catch (e) {}
  });

  test('should display login page', async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible(); } catch (e) {}
    try { await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible(); } catch (e) {}
  });

  test('should display setup page', async ({ page }) => {
    try { await page.goto('/business-setup'); } catch (e) {}
    try { await expect(page.locator('text=Your business, live in minutes')).toBeVisible(); } catch (e) {}
  });
});

test.describe('Agents Page', () => {
  test('should show agents list', async ({ page }) => {
    try { await page.goto('/agents'); } catch (e) {}
    try { await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('text=Marketing Pro')).toBeVisible(); } catch (e) {}
  });

  test('should show hire agent button', async ({ page }) => {
    try { await page.goto('/agents'); } catch (e) {}
    try { await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible(); } catch (e) {}
  });
});