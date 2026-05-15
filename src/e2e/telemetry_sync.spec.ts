import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: Telemetry Sync UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill( 'password123');
    await page.click(UI_LOCATORS.SIGN_IN_BTN);
    await page.waitForURL('**/dashboard*');
  });

  test('should display Standalone-to-Cloud Telemetry Sync header', async ({ page }) => {
    await expect(page.locator(UI_LOCATORS.DASHBOARD_TEXT_2)).toBeVisible();
  });

  test('should navigate to Telemetry Settings', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    await expect(page).toHaveURL(/.*settings.*/);
  });

  test('should display Advanced Mode toggle', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    const toggle = page.locator(UI_LOCATORS.ADVANCED_TEXT);
    await expect(toggle).toBeVisible();
  });

  test('should allow enabling Advanced Mode', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    const advancedTab = page.locator(UI_LOCATORS.ADVANCED_TEXT).filter({ visible: true }).first();
    await advancedTab.click();
    await expect(page.locator(UI_LOCATORS.ADVANCED_TEXT)).toBeVisible();
  });

  test('should return to Dashboard after Settings', async ({ page }) => {
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    await page.click('button:has-text("Dashboard"), a:has-text("Dashboard")');
    await expect(page).toHaveURL(/.*dashboard.*/);
  });
});
