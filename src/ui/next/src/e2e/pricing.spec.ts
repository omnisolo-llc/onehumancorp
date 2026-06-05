import { test, expect } from '@playwright/test';

test.describe('Pricing Page', () => {
  test('Pricing page loads and displays plans', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Navigate to the pricing page
    await page.goto('http://localhost:3000/pricing');

    // Wait for the main heading to appear
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });

    // Check that the tier names exist
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Check navigation works
    await page.locator('button', { hasText: 'Back to Dashboard' }).click();
    await expect(page).toHaveURL('http://localhost:3000/dashboard');
  });
});
