import { test, expect } from '@playwright/test';

test.describe('Miser Role Pricing and Billing Features', () => {

  test('Cost Dashboard displays cost transparency', async ({ page }) => {
    // Navigate to the Cost Dashboard directly
    await page.goto('/cost-dashboard');

    // Wait for it to load
    await expect(page.locator('text=Advisory Summary')).toBeVisible({ timeout: 10000 });

    // Check key elements
    await expect(page.locator('text=Cost Transparency')).toBeVisible();
    await expect(page.locator('text=Total Costs')).toBeVisible();
    await expect(page.locator('text=Cost Breakdown')).toBeVisible();

    // Verify fallback mock data if real API is missing or fails,
    // we added route mock but let's check for currency symbol
    await expect(page.locator('text=$').first()).toBeVisible();
  });

  test('My Plan page displays current plan and limits', async ({ page }) => {
    // Navigate to the My Plan page
    await page.goto('/plan');

    // Wait for it to load
    await expect(page.locator('h1:has-text("My Plan")')).toBeVisible({ timeout: 10000 });

    // Verify key sections exist
    await expect(page.locator('text=Current Plan')).toBeVisible();
    await expect(page.locator('text=Estimated Next Bill')).toBeVisible();
    await expect(page.locator('text=Your Current Usage')).toBeVisible();
    await expect(page.locator('text=AI Actions Used')).toBeVisible();
    await expect(page.locator('text=Storage Used')).toBeVisible();
  });

  test('Pricing page displays tiers and allows upgrade via Stripe mock', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');

    // Wait for page
    await expect(page.locator('h1:has-text("Pricing Plans")')).toBeVisible({ timeout: 10000 });

    // Verify tiers
    await expect(page.locator('h3:has-text("Free")')).toBeVisible();
    await expect(page.locator('h3:has-text("Starter")')).toBeVisible();
    await expect(page.locator('h3:has-text("Pro")')).toBeVisible();
    await expect(page.locator('h3:has-text("Business")')).toBeVisible();

    // Click to upgrade to Starter
    await page.locator('button:has-text("Upgrade to Starter via Stripe")').click();

    // Should navigate to checkout with tier query
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);

    // In checkout
    await expect(page.locator('h1:has-text("Checkout")')).toBeVisible();

    // Click pay now
    await page.locator('button:has-text("Pay Now")').click();

    // Should show success modal
    await expect(page.locator('text=Payment Successful!')).toBeVisible({ timeout: 5000 });
  });
});
