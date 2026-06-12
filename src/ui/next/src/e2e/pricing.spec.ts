import { test, expect } from '@playwright/test';

test.describe('Pricing Page Loop', () => {
  test('Pricing page loads and displays tiers correctly', async ({ page }) => {
    // Navigate to the pricing page
    await page.goto('/pricing');
    await page.waitForResponse(response => response.url().includes('/api/billing/pricing-plans') && response.status() === 200);

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });

    // Check that all tiers are displayed
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Check that Upgrade buttons are present
    await expect(page.locator('button', { hasText: 'Upgrade to Starter' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Pro' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Business' })).toBeVisible();

    // Check navigation works
    await page.locator('button', { hasText: 'Back' }).click();
    await expect(page).toHaveURL('/dashboard');
  });
});
