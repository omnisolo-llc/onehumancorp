import { test, expect } from './fixtures';

test.describe('Cost Engineering & Billing UI', () => {
  // `page` fixture is already authenticated as admin
  test('Admin can view My Plan, check Cost details, and open Pricing to upgrade', async ({ page }) => {
    // Wait for dashboard to fully load
    await expect(page.locator('#dashboard-screen')).toBeVisible();

    // The user opens the side nav (on mobile/desktop) and clicks Billing
    await page.locator('button', { hasText: 'Menu' }).click();
    await page.locator('button', { hasText: 'Billing' }).click();

    // Wait for the My Plan screen to fetch and render
    await expect(page.locator('#my-plan-screen')).toBeVisible();
    await expect(page.locator('#my-plan-name')).toContainText('Plan: Free');

    // Check usage sections are visible
    await expect(page.locator('text=AI Actions Used')).toBeVisible();
    await expect(page.locator('text=Storage Used')).toBeVisible();

    // Navigate to Cost Details
    await page.locator('.card.glass.hover-glass', { hasText: 'View Cost Details' }).click();

    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.locator('h1', { hasText: 'Cost Transparency' })).toBeVisible();

    // Check some breakdown items
    await expect(page.locator('text=LLM Usage')).toBeVisible();
    await expect(page.locator('text=Storage & CDN')).toBeVisible();

    // Navigate back to My Plan
    await page.locator('button', { hasText: 'Back to My Plan' }).click();
    await expect(page.locator('#my-plan-screen')).toBeVisible();

    // Navigate to Pricing from My Plan
    await page.locator('button', { hasText: 'View Upgrade Plans' }).click();

    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Check pricing tiers are rendered correctly
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Verify upgrade buttons exist
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();

    // Mock a click on checkout
    await upgradeButton.click();
    await expect(page.locator('#checkout-screen')).toBeVisible();
  });
});
