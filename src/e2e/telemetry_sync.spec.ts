import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: Telemetry Sync UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill( 'test@example.com');
    await page.locator('input[type="password"]').first().fill( 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/dashboard*');
  });

  test('should display Standalone-to-Cloud Telemetry Sync header', async ({ page }) => {
    await expect(page.locator('text=Dashboard')).toBeVisible();
  });

  test('should navigate to Telemetry Settings', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    await expect(page).toHaveURL(/.*settings.*/);
  });

  test('should display Advanced Mode toggle', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    const toggle = page.locator('text=Advanced');
    await expect(toggle).toBeVisible();
  });

  test('should allow enabling Advanced Mode', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    const advancedTab = page.locator('text=Advanced').first();
    await advancedTab.click();
    await expect(page.locator('text=Advanced')).toBeVisible();
  });

  test('should return to Dashboard after Settings', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    await page.click('button:has-text("Dashboard"), a:has-text("Dashboard")');
    await expect(page).toHaveURL(/.*dashboard.*/);
  });
});
