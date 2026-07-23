import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Cost Dashboard and Pricing UI', () => {
  test('should display pricing page successfully', async ({ page }) => {
    await adminPage(page);
    await page.goto('/pricing.html');
    await expect(page.locator('h1').first()).toContainText('Pricing Plans');
    await expect(page.locator('.plan-name').nth(0)).toContainText('Free');
    await expect(page.locator('.plan-name').nth(1)).toContainText('Starter');
    await expect(page.locator('.plan-name').nth(2)).toContainText('Pro');
    await expect(page.locator('.plan-name').nth(3)).toContainText('Business');
  });

  test('should display cost dashboard and navigate to pricing', async ({ page }) => {
    await adminPage(page);
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('h1').first()).toContainText('My Plan');
    const upgradeButton = page.locator('button:has-text("Upgrade")').first();
    await upgradeButton.click();
    await page.waitForURL('**/pricing.html*');
  });

  test('should toggle to cost transparency view without reload', async ({ page }) => {
    await adminPage(page);
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('#my-plan-widget')).toBeVisible();

    const viewDetailedCostsBtn = page.locator('#view-detailed-costs');
    await viewDetailedCostsBtn.click();

    await expect(page.locator('#cost-dashboard-widget')).toBeVisible();
    await expect(page.locator('#my-plan-widget')).toBeHidden();

    // Check elements
    await expect(page.locator('h1').filter({ hasText: 'Cost Transparency Dashboard' })).toBeVisible();

    const backBtn = page.locator('#back-to-my-plan');
    await backBtn.click();

    await expect(page.locator('#my-plan-widget')).toBeVisible();
    await expect(page.locator('#cost-dashboard-widget')).toBeHidden();
  });
});
