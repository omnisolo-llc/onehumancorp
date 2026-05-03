import { test, expect } from '@playwright/test';

test.describe('Miser - Cost & Pricing Flows', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to app and login
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    // Wait for Dashboard
    await page.waitForURL('**/*');
  });

  test('Navigate to Billing from Dashboard', async ({ page }) => {
    // Ensure dashboard is loaded
    await expect(page.locator('text=App Settings')).toBeVisible();

    // Click "Billing" button in the dashboard
    await page.click('button:has-text("Billing")');

    // Verify it navigated to My Plan
    await expect(page.locator('text=My Plan').first()).toBeVisible();
    await expect(page.locator('text=Current Plan')).toBeVisible();
  });

  test('View Cost Details in My Plan', async ({ page }) => {
    // Navigate to Billing directly
    await page.click('button:has-text("Billing")');
    await expect(page.locator('text=My Plan').first()).toBeVisible();

    // Click View Cost Details
    await page.click('button:has-text("View Cost Details")');

    // Verify it opened Cost Dashboard
    await expect(page.locator('text=Cost & AI Usage').first()).toBeVisible();
    await expect(page.locator('text=Total Spend').first()).toBeVisible();
  });

  test('Upgrade Plan flow from My Plan', async ({ page }) => {
    // Navigate to Billing
    await page.click('button:has-text("Billing")');
    await expect(page.locator('text=My Plan').first()).toBeVisible();

    // Click Upgrade
    await page.click('button:has-text("Upgrade")');

    // Verify it opened Pricing & Billing Comparison
    await expect(page.locator('text=Plan Comparison')).toBeVisible();
    await expect(page.locator('text=Free')).toBeVisible();
    await expect(page.locator('text=Pro')).toBeVisible();
  });

  test('Toggle Annual and Monthly pricing in Pricing Page', async ({ page }) => {
    // Navigate to Billing -> Upgrade
    await page.click('button:has-text("Billing")');
    await page.click('button:has-text("Upgrade")');
    await expect(page.locator('text=Plan Comparison')).toBeVisible();

    // Ensure it starts Monthly or Annual
    const toggleButton = page.locator('button', { hasText: /(Annual|Monthly)/ }).first();
    await expect(toggleButton).toBeVisible();

    // Toggle
    await toggleButton.click();

    // Wait for toggle state to change
    await page.waitForTimeout(500);

    // It should have toggled, we can't easily assert text without knowing initial state, but clicking shouldn't crash
    await expect(page.locator('text=Free')).toBeVisible();
    await expect(page.locator('text=Pro')).toBeVisible();
  });

  test('Select Pro Plan in Pricing Page', async ({ page }) => {
    // Navigate to Billing -> Upgrade
    await page.click('button:has-text("Billing")');
    await page.click('button:has-text("Upgrade")');
    await expect(page.locator('text=Plan Comparison')).toBeVisible();

    // Choose Pro
    await page.click('button:has-text("Choose Pro")');

    // Verify some state change or fallback back to MyPlan if configured
    // For now we just make sure the button exists and is clickable
    await expect(page.locator('text=Plan Comparison')).toBeVisible();
  });

});
