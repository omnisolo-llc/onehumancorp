import { test, expect } from '@playwright/test';

test.describe('Pricing Page CUJ', () => {
  test('Owner navigates to pricing and selects a plan', async ({ page }) => {
    // 1. Owner starts at dashboard
    await page.goto('http://localhost:3000/dashboard');

    // 2. Navigate to pricing page
    await page.goto('http://localhost:3000/pricing');

    // 3. Verify page loads and title is visible
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });

    // 4. Verify all tiers are present
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // 5. Select Starter Plan
    await page.locator('button', { hasText: 'Upgrade to Starter via Stripe' }).click();

    // 6. Verify navigation to checkout
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);
  });
});
