import { test, expect } from '@playwright/test';

test.describe('Services Billing & Pricing CUJ', () => {
  test('User can view pricing plans and click upgrade on Starter tier', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');

    // Ensure all pricing tiers are visible
    await expect(page.locator('text=Free')).toBeVisible();
    await expect(page.locator('text=Starter')).toBeVisible();
    await expect(page.locator('text=Pro')).toBeVisible();
    await expect(page.locator('text=Business')).toBeVisible();

    // Check that the recommended starter plan has the correct price and button
    await expect(page.locator('text=$29')).toBeVisible();

    // Click the upgrade button for the Starter plan
    await page.click('button:has-text("Upgrade to Starter via Stripe")');

    // Verify it redirects to the checkout page with the correct tier parameter
    await expect(page).toHaveURL(/\/checkout\?tier=Starter/);

    // Verify the checkout page shows the correct tier
    await expect(page.locator('body')).toContainText('Starter');
  });
});
