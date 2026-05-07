import { test, expect } from '@playwright/test';

test.describe('Shipping Fulfillment Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // From root, navigate to Login
    await page.locator('text=/Login/i').click();
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');

    // Wait for the Dashboard to load and then click the newly added Fulfill button
    const fulfillBtn = page.locator('button:has-text("Fulfill"), text="Fulfill"').first();
    await fulfillBtn.waitFor({ state: 'visible' });
    await fulfillBtn.click();
  });

  test('should display shipping fulfillment header', async ({ page }) => {
    await expect(page.locator('text=/Fulfill Order|Fulfill & Ship via Shippo/i').first()).toBeVisible();
  });

  test('should show customer and destination info', async ({ page }) => {
    await expect(page.locator('text=/Shipping to:/i').first()).toBeVisible();
  });

  test('should present available shipping rates', async ({ page }) => {
    // Tests that at least one shipping rate is available (like USPS Priority)
    await expect(page.locator('text=/USPS Priority|UPS Ground|FedEx Express/i').first()).toBeVisible();
  });

  test('should enable purchase label button upon rate selection', async ({ page }) => {
    // Click a rate card
    const rateCard = page.locator('text=/USPS Priority|UPS Ground|FedEx Express/i').first();
    await rateCard.waitFor({ state: 'visible' });
    await rateCard.click();
    const purchaseBtn = page.locator('text=/Purchase Label/i').first();
    await expect(purchaseBtn).toBeVisible();
    await expect(purchaseBtn).toBeEnabled();
  });

  test('should complete purchase and show print label options', async ({ page }) => {
    const rateCard = page.locator('text=/USPS Priority|UPS Ground|FedEx Express/i').first();
    await rateCard.waitFor({ state: 'visible' });
    await rateCard.click();
    const purchaseBtn = page.locator('text=/Purchase Label/i').first();
    await purchaseBtn.click();

    // Step 1 assertions
    await expect(page.locator('text=/Label Purchased Successfully!/i').first()).toBeVisible();
    await expect(page.locator('text=/Print Label PDF/i').first()).toBeVisible();
    await expect(page.locator('text=/Done/i').first()).toBeVisible();
  });
});
