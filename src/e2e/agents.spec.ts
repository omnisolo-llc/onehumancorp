import { test, expect } from './fixtures';

test.describe('Agent Management', () => {
  test('should display agents page (My Staff)', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'My Staff' })).toBeVisible();
  });

  test('should show The Ambassador department', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=Customer Success')).toBeVisible();
  });

  test('should show The Manager department', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=The Manager')).toBeVisible();
    await expect(page.locator('text=Operations')).toBeVisible();
  });

  test('should show The Salesperson department', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=The Salesperson')).toBeVisible();
    await expect(page.locator('text=Sales')).toBeVisible();
  });

  test('should toggle department settings', async ({ page }) => {
    await page.goto('/agents');
    const ambassadorCard = page.locator('text=The Ambassador').locator('..');

    // Settings should be hidden initially
    await expect(page.locator('text=Require approval for quotes > $100')).not.toBeVisible();

    // Click card to show settings
    await ambassadorCard.click();
    await expect(page.locator('text=Require approval for quotes > $100')).toBeVisible();

    // Click checkbox
    await page.locator('text=Require approval for quotes > $100').locator('input[type="checkbox"]').uncheck();

    // Wait for the alert
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Settings updated for ambassador');
      await dialog.accept();
    });
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});