import { test, expect } from './fixtures';

test.describe('Lens Audit Visual Checks', () => {
  test('should display Expert Center heading on agents page', async ({ page }) => {
    // Navigate via proper path to Tauri/Rust embedded UI which has agents logic
    await page.goto('/agents');
    await page.waitForTimeout(2000);
    await expect(page.locator('text=Expert Center').first()).toHaveCount(1);
  });

  test('should navigate to dashboard and show welcome message', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });

  test('should display dashboard correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator("h1.app-title", { hasText: "Dashboard" })).toBeVisible();
    await expect(page.getByRole('navigation').first()).toBeVisible();
  });

  test('should navigate to website builder', async ({ page }) => {
    await page.goto('/website-builder');
    await page.waitForTimeout(2000);
    await expect(page.getByRole('heading', { name: 'Setup Assistant' }).first()).toBeVisible();
  });

  test('should display login fields', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole("textbox", { name: "Email or username" }).filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Log in' })).toBeVisible();
  });
});
