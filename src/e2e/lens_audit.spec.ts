import { test, expect } from './fixtures';

test.describe('Lens Audit Visual Checks', () => {
  test('should display Expert Center heading on agents page', async ({ page }) => {
    // Navigate via proper path to Tauri/Rust embedded UI which has agents logic
    await page.goto('http://127.0.0.1:3000/agents');
    await page.waitForTimeout(2000);
    await expect(page.locator('text=Expert Center').first()).toHaveCount(1);
  });

  test('should navigate to dashboard and show welcome message', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });

  test('should display dashboard correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('navigation').first()).toBeVisible();
  });

  test('should navigate to website builder', async ({ page }) => {
    await page.goto('http://127.0.0.1:3000/website-builder');
    await page.waitForTimeout(2000);
    // click the assistant button
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
  });

  test('should display login fields', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Log In' })).toBeVisible();
  });
});
