import { test, expect } from '@playwright/test';

test.describe('Maya Dashboard Interaction Audit - 375px', () => {
  test.beforeEach(async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.addInitScript(() => {
      window.localStorage.setItem('has_onboarded', 'true');
      window.localStorage.setItem('user_name', 'Maya');
      window.localStorage.setItem('tenant_id', 'maya-bakes');
    });
    await page.goto('http://localhost:3000/dashboard');
    // Ensure we are on the dashboard
    await page.waitForSelector('h2:has-text("Welcome back")');
  });

  test('Start Tour button opens walkthrough', async ({ page }) => {
    const tourBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(tourBtn).toBeVisible();
    await tourBtn.click();
    // Check if walkthrough content appears
    await expect(page.locator('.ohc-walkthrough-bubble h4:has-text("Business Analytics")')).toBeVisible();
    await expect(page.locator('text=This panel reads sales')).toBeVisible();
  });

  test('Launch Site button navigates to onboarding (as redirect from business-setup)', async ({ page }) => {
    const launchBtn = page.locator('button:has-text("Launch Site")');
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();
    await expect(page).toHaveURL(/.*onboarding/);
  });

  test('Migrate Existing Store button toggles section', async ({ page }) => {
    const migrateBtn = page.locator('button:has-text("Migrate Existing Store")');
    await expect(migrateBtn).toBeVisible();
    await migrateBtn.click();
    await expect(page.locator('text=Store Migration')).toBeVisible();
  });

  test('Business Analytics link navigates correctly', async ({ page }) => {
    const analyticsLink = page.locator('a:has-text("Business Analytics")').first();
    await expect(analyticsLink).toBeVisible();
    await analyticsLink.click();
    await expect(page).toHaveURL(/.*business-analytics/);
  });

  test('Open Orders link navigates correctly', async ({ page }) => {
    const ordersLink = page.locator('a:has-text("Open Orders")');
    await expect(ordersLink).toBeVisible();
    await ordersLink.click();
    await expect(page).toHaveURL(/.*orders/);
  });

  test('Assistant Tasks card navigates correctly', async ({ page }) => {
    const assistantCard = page.locator('h3:has-text("Assistant Tasks")');
    await expect(assistantCard).toBeVisible();
    await page.locator('a:has(h3:has-text("Assistant Tasks"))').click();
    await expect(page).toHaveURL(/.*assistant/);
  });

  test('Inventory link navigates correctly', async ({ page }) => {
    const inventoryLink = page.locator('section a:has-text("Inventory")');
    await expect(inventoryLink).toBeVisible();
    await inventoryLink.click();
    await expect(page).toHaveURL(/.*inventory/);
  });
});
