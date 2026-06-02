import { test, expect } from './fixtures';

test.describe('Pricing UI test', () => {
  test('Owner navigates to pricing and sees all tiers', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Evaluate javascript to change screen to pricing
    await page.evaluate(() => {
        // @ts-ignore
        window.showScreen('pricing-screen');
    });

    // Check pricing screen
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' }).first()).toBeVisible();

    // Verify all tiers are visible
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();

    // Verify buttons
    await expect(page.getByRole('button', { name: 'Current Plan' })).toBeVisible();

    // There are multiple Upgrade buttons
    const upgradeButtons = page.locator('button:has-text("Upgrade via Stripe")');
    await expect(upgradeButtons).toHaveCount(3);
  });
});
