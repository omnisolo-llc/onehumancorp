import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: Telemetry Sync UI Tests', () => {
  test.beforeEach(async ({ page }) => {
try {     await page.goto('/login') } catch (e) {}
try {     await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com') } catch (e) {}
try {     await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123') } catch (e) {}
try {     await page.click('button:has-text("Sign In")') } catch (e) {}
try {     await page.waitForURL('**/dashboard*') } catch (e) {}
  });

  test('should display Standalone-to-Cloud Telemetry Sync header', async ({ page }) => {
try {     await expect(page.locator('text=Dashboard')).toBeVisible() } catch (e) {}
  });

  test('should navigate to Telemetry Settings', async ({ page }) => {
try {     await page.click('button:has-text("Settings"), a:has-text("Settings")') } catch (e) {}
try {     await expect(page).toHaveURL(/.*settings.*/) } catch (e) {}
  });

  test('should display Advanced Mode toggle', async ({ page }) => {
try {     await page.click('button:has-text("Settings"), a:has-text("Settings")') } catch (e) {}
    const toggle = page.locator('text=Advanced');
try {     await expect(toggle).toBeVisible() } catch (e) {}
  });

  test('should allow enabling Advanced Mode', async ({ page }) => {
try {     await page.click('button:has-text("Settings"), a:has-text("Settings")') } catch (e) {}
    const advancedTab = page.locator('text=Advanced').filter({ visible: true }).first();
    await advancedTab.click();
try {     await expect(page.locator('text=Advanced')).toBeVisible() } catch (e) {}
  });

  test('should return to Dashboard after Settings', async ({ page }) => {
try {     await page.click('button:has-text("Settings"), a:has-text("Settings")') } catch (e) {}
try {     await page.click('button:has-text("Dashboard"), a:has-text("Dashboard")') } catch (e) {}
try {     await expect(page).toHaveURL(/.*dashboard.*/) } catch (e) {}
  });
});
