import { test, expect } from './fixtures';

test.describe('Pricing UI Loop', () => {
  test('Pricing page loads and displays data', async ({ page }) => {
    // Navigate to the pricing page
    await page.goto('/pricing');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });

    // Check that all tiers are displayed
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Check that buttons are present
    await expect(page.locator('button', { hasText: 'Current Plan' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Starter via Stripe' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Pro via Stripe' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Business via Stripe' })).toBeVisible();

    // Check that the tooltip is present
    await expect(page.locator('h2', { hasText: 'Frequently Asked Questions' })).toBeVisible();

    // Check navigation works
    await page.locator('button', { hasText: 'Back to Dashboard' }).click();
    await expect(page).toHaveURL('/dashboard');
  });
});
