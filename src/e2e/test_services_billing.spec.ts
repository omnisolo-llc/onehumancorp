import { test, expect } from '@playwright/test';

test.describe('Services Billing & Pricing CUJ', () => {
  test('User can view pricing plans and click upgrade on Starter tier', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');

    // Ensure all pricing tiers are visible
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();

    // Check that the recommended starter plan has the correct price and button
    await expect(page.getByText('$29 / month')).toBeVisible();

    // Click the upgrade button for the Starter plan
    await page.click('button:has-text("Upgrade to Starter via Stripe")');

    // Verify it redirects to the checkout page
    await expect(page).toHaveURL(/.*\/checkout.*/);
  });
});
