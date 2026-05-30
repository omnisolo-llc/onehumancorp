import { test, expect } from './fixtures';

test.describe('Cost Engineering & Billing UI', () => {
  test('Admin can view My Plan, check Cost details, and open Pricing to upgrade', async ({ page, adminPage }) => {
    // Navigate to Dashboard
    await adminPage.goto('/dashboard');
    await expect(adminPage.locator('#dashboard-screen')).toBeVisible();

    // The user opens the side nav (on mobile/desktop) and clicks Billing
    await adminPage.locator('button', { hasText: 'Menu' }).click();
    await adminPage.locator('button', { hasText: 'Billing' }).click();

    // Wait for the My Plan screen to fetch and render
    await expect(adminPage.locator('#my-plan-screen')).toBeVisible();
    await expect(adminPage.locator('#my-plan-name')).toContainText('Plan: Free');

    // Check usage sections are visible
    await expect(adminPage.locator('text=AI Actions Used')).toBeVisible();
    await expect(adminPage.locator('text=Storage Used')).toBeVisible();

    // Navigate to Cost Details
    await adminPage.locator('.card.glass.hover-glass', { hasText: 'View Cost Details' }).click();

    await expect(adminPage.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(adminPage.locator('h1', { hasText: 'Cost Transparency' })).toBeVisible();

    // Check some breakdown items
    await expect(adminPage.locator('text=LLM Usage')).toBeVisible();
    await expect(adminPage.locator('text=Storage & CDN')).toBeVisible();

    // Navigate back to My Plan
    await adminPage.locator('button', { hasText: 'Back to My Plan' }).click();
    await expect(adminPage.locator('#my-plan-screen')).toBeVisible();

    // Navigate to Pricing from My Plan
    await adminPage.locator('button', { hasText: 'View Upgrade Plans' }).click();

    await expect(adminPage.locator('#pricing-screen')).toBeVisible();
    await expect(adminPage.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Check pricing tiers are rendered correctly
    await expect(adminPage.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(adminPage.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(adminPage.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Verify upgrade buttons exist
    const upgradeButton = adminPage.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();

    // Mock a click on checkout
    await upgradeButton.click();
    await expect(adminPage.locator('#checkout-screen')).toBeVisible();
  });
});
