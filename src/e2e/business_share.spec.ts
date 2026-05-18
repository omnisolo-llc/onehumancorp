import { test, expect } from './fixtures';

test.describe('Business Share & Embed', () => {
  test('should display dashboard with nav links', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
    await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible();
    await expect(page.locator('nav a:has-text("Agents")')).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/?dashboard=1');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
  });

  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.locator('text=Marketing Pro')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });
});