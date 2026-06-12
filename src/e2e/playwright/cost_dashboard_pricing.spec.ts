import { test, expect } from '@playwright/test';

test.describe('Cost Dashboard and Pricing Pages', () => {
  // Test user needs to login or use a session
  // Since we don't have standard auth login documented here, we rely on the pages being accessible
  // or use test credentials if needed. For this test we assume standard access like other e2e tests.
  // Many OHC pages seem accessible in e2e without strict auth setup if using local dev.

  test('Cost Dashboard renders correctly', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Expect "Cost Transparency" header
    await expect(page.locator('text=Cost Transparency')).toBeVisible({ timeout: 10000 });

    // Expect "My Plan" section
    await expect(page.locator('text=My Plan')).toBeVisible();

    // Expect 7-Day Trend
    await expect(page.locator('text=7-Day Trend')).toBeVisible();

    // Expect Agent & Feature Costs
    await expect(page.locator('text=Agent & Feature Costs')).toBeVisible();
  });

  test('Pricing page renders correctly and links to checkout', async ({ page }) => {
    await page.goto('/pricing');

    // Expect "Pricing Plans" header
    await expect(page.locator('text=Pricing Plans')).toBeVisible({ timeout: 10000 });

    // Expect specific tiers
    await expect(page.locator('text=Free')).toBeVisible();
    await expect(page.locator('text=Starter')).toBeVisible();
    await expect(page.locator('text=Pro')).toBeVisible();
    await expect(page.locator('text=Business')).toBeVisible();

    // Click upgrade to starter
    await page.click('button:has-text("Upgrade to Starter")');

    // Expect navigation to checkout
    await expect(page).toHaveURL(/\/checkout\?tier=Starter/);
  });
});
